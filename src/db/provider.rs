use log::{error, info, warn};
use tokio::task::spawn;
use tokio_postgres::{Client, NoTls};

use crate::popis_error::{PopisError, Result};

const HOST_ENV_VAR: &str = "POSTGRESHOST";
const PORT_ENV_VAR: &str = "POSTGRESPORT";

pub struct Provider {
    pub(super) client: Client,
}

impl Provider {
    pub async fn new() -> Result<Provider> {
        let host = std::env::var(HOST_ENV_VAR).unwrap_or_else(|_e| {
            warn!("Postgres host variable invalid or not provided");
            String::from("localhost")
        });
        let port = std::env::var(PORT_ENV_VAR).unwrap_or_else(|_e| {
            warn!("Posgres port variable invalid or not provided");
            String::from("5432")
        });
        let config = format!("host={host} port={port} user=postgres password=admin");
        info!("Connecting to postres using {config}");
        let (client, connection) = tokio_postgres::connect(&config, NoTls)
            .await
            .map_err(|e| PopisError::DbConnectionError(e.to_string()))?;
        spawn(async move {
            if let Err(e) = connection.await {
                error!("Error running the database: {e}");
            }
        });
        init_tables(&client).await?;
        Ok(Provider { client })
    }
}

async fn init_tables(db: &Client) -> Result<()> {
    for statement in [
        "CREATE TABLE IF NOT EXISTS seating (
            id SERIAL PRIMARY KEY,
            date timestamptz NOT NULL,
            identifier INT NOT NULL,
            UNIQUE(identifier)
        );",
        "CREATE TABLE IF NOT EXISTS voting (
            id SERIAL PRIMARY KEY,
            identifier INT NOT NULL,
            seating_id INT NOT NULL,
            description VARCHAR NOT NULL,
            UNIQUE(identifier),
            CONSTRAINT fk_seating
            FOREIGN KEY(seating_id) 
            REFERENCES seating(id)
            ON DELETE CASCADE
        );",
        "CREATE TABLE IF NOT EXISTS party (
            id SERIAL PRIMARY KEY,
            name VARCHAR UNIQUE NOT NULL
        );",
        "CREATE TABLE IF NOT EXISTS vote (
            id SERIAL PRIMARY KEY,
            voting_id INT NOT NULL,
            party_id INT NOT NULL,
            result INT NOT NULL,
            CONSTRAINT fk_voting_result
            FOREIGN KEY(voting_id) 
            REFERENCES voting(id)
            ON DELETE CASCADE,
            CONSTRAINT fk_party
            FOREIGN KEY(party_id) 
            REFERENCES party(id)
            ON DELETE CASCADE
        );"
    ] {
        db.execute(statement, &[])
            .await
            .map_err(|e| PopisError::DbCommunicationError(format!("Couldn't init tables: {e}")))?;
    }
    Ok(())
}
