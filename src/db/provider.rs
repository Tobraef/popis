use log::{warn, info, error};
use tokio_postgres::{Client, NoTls};
use tokio::task::spawn;

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
        let config = format!("host={host} port={port} user=postgres");
        info!("Connecting to postres using {config}");
        let (client, connection) = tokio_postgres::connect(&config, NoTls).await
            .map_err(|e| PopisError::DbConnectionError(e.to_string()))?;
        spawn(async move {
           if let Err(e) = connection.await {
            error!("Error running the database: {e}");
           } 
        });
        Ok(Provider { client })
    }
}