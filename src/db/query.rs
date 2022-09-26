use rand::Rng;
use tokio_postgres::Client;

use crate::domain::{SeatingHeader, Voting, VotingResult, PartyVote, Party, VotingHeader};
use crate::popis_error::{PopisError, Result};

use super::provider::Provider;

pub async fn contains_seating(provider: &Provider, seating: &SeatingHeader) -> Result<bool> {
    let db = &provider.client;
    Ok(!db
        .query(
            "SELECT 1 FROM seating WHERE identifier = $1",
            &[&seating.identifier],
        )
        .await
        .map_err(|e| {
            PopisError::DbCommunicationError(format!("Error checking unique seating: {}", e))
        })?
        .is_empty())
}

pub(super) async fn raw_parties_except(
    provider: &Provider,
    to_exclude: &[&str],
) -> Result<impl Iterator<Item = (String, i32)>> {
    let db = &provider.client;
    Ok(db
        .query(
            &format!(
                " 
        SELECT name, id
        FROM party
        WHERE name IN ({})",
                to_exclude.join(",")
            ),
            &[],
        )
        .await
        .map_err(|e| PopisError::DbConnectionError(e.to_string()))?
        .into_iter()
        .map(|r| (r.get(0), r.get(1))))
        compile_error!("something wrong with the IN query")
}

async fn max_seating_identifier(db: &Client) -> Result<i32> {
    db
        .query_one("SELECT MAX(identifier) FROM seating", &[])
        .await
        .map(|r| r.get(0))
        .map_err(|e| PopisError::DbCommunicationError(format!("Error selecting max seating identifier: {}", e)))
}

async fn max_voting_identifier(seating_identifier: i32, db: &Client) -> Result<i32> {
    db
        .query_one("SELECT MAX(identifier) FROM voting WHERE seating_id = (SELECT id FROM seating WHERE identifier = $1)", &[&seating_identifier])
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Error selecting max seating identifier: {}", e)))
        .map(|r| r.get(0))
}

pub async fn random_voting(provider: &Provider) -> Result<Voting> {
    let db = &provider.client;
    let max_seating = max_seating_identifier(db).await?;
    let random_seating_identifier = rand::thread_rng().gen_range(1..=max_seating);
    let max_voting_identifier = max_voting_identifier(random_seating_identifier, db).await?;
    let random_voting_identifier = rand::thread_rng().gen_range(1..=max_voting_identifier);
    let mut description = None;
    let results = db
        .query(
            "SELECT v.description, p.name, r.result 
                        FROM voting v 
                        INNER JOIN vote r ON v.id = r.voting_id 
                        INNER JOIN party p ON p.id = r.party_id
                        WHERE v.identifier = $1", &[&random_voting_identifier])
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Error selecting random voting: {}", e)))
        .map(|rows| VotingResult::new(rows
            .into_iter()
            .map(|row| {
                description.get_or_insert(row.get(0));
                PartyVote::new(Party::new(row.get(1)), row.get::<usize,i32>(2).into())
            })
            .collect()
        ))?;
    Ok(Voting::new(VotingHeader::new(random_voting_identifier, description.unwrap()), results))

}