use rand::Rng;
use tokio_postgres::Client;

use crate::domain::{SeatingHeader, PartyVote, Party, VotingHeader};
use crate::popis_error::{PopisError, Result};

use super::data::DbVotingHeader;
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

pub(super) async fn parties(
    provider: &Provider,
) -> Result<impl Iterator<Item = (String, i32)>> {
    let db = &provider.client;
    Ok(db
        .query("SELECT name, id FROM party;", &[])
        .await
        .map_err(|e| PopisError::DbConnectionError(e.to_string()))?
        .into_iter()
        .map(|r| (r.get(0), r.get(1))))
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

pub async fn random_voting_header(provider: &Provider) -> Result<DbVotingHeader> {
    let db = &provider.client;
    let max_seating = max_seating_identifier(db).await?;
    let random_seating_identifier = rand::thread_rng().gen_range(1..=max_seating);
    let max_voting_identifier = max_voting_identifier(random_seating_identifier, db).await?;
    let random_voting_identifier = rand::thread_rng().gen_range(1..=max_voting_identifier);
    db
        .query_one(
            "SELECT v.description, v.id, v.identifier 
                        FROM voting v 
                        WHERE v.identifier = $1 and v.seating_id = (SELECT id FROM seating s WHERE s.identifier = $2);", &[&random_voting_identifier, &random_seating_identifier])
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Error selecting random voting: {}", e)))
        .map(|row| DbVotingHeader::new(row.get(1), VotingHeader::new(row.get(2), row.get(0))))
}

pub async fn voting_results(provider: &Provider, voting_id: i32) -> Result<Vec<PartyVote>> {
    let db = &provider.client;
    db
        .query("SELECT p.name, v.vote 
                           FROM voting vs
                           INNER JOIN vote v ON vs.id = v.voting_id 
                           INNER JOIN party p ON p.id = v.party_id 
                           WHERE vs.id = $1", &[&voting_id])
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Error selecting voting results: {}", e)))
        .map(|rows| rows
            .into_iter()
            .map(|row| PartyVote::new(Party::new(row.get(0)), row.get::<usize,i32>(1).into()))
            .collect())
}