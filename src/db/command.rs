use std::fmt::Debug;

use postgres::{types::ToSql, Error};

use super::provider::Provider;
use crate::{popis_error::{Result, PopisError}, domain::{Seating, Voting, VotingResult}};

pub async fn init_tables(provider: &Provider) -> Result<()> {
    let db = &provider.client;
    db.execute(r#"
    CREATE TABLE IF NOT EXISTS seating (
        id SERIAL PRIMARY KEY,
        date TIMESTAMP WITH TIMEZONE NOT NULL,
        number UNIQUE NOT NULL,
    );
    CREATE TABLE IF NOT EXISTS voting (
        id SERIAL PRIMARY KEY,
        number UNIQUE NOT NULL,
        seating_id INT NOT NULL,
        description VARCHAR NOT NULL,
        CONSTRAINT fk_seating
        FOREIGN KEY(seating_id) 
        REFERENCES seating(id)
        ON DELETE CASCADE
    );
    CREATE TABLE IF NOT EXISTS party (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL
    );
    CREATE TABLE IF NOT EXISTS vote (
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
    );"#, &[])
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Couldn't init tables: {e}")))
        .map(|_| ())
}

async fn insert_query(provider: &Provider, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<i64> 
{
    let db = &provider.client;
    db.query_one(query, params)
        .await
        .map_err(|e| PopisError::DbCommunicationError(format!("Couldn't insert seating with {params:?} into db: {e}")))
        .map(|r| r.get(0))    
}

fn verify_if_contains_data(seating: &Seating) -> Result<()> {
    if let Some(votings) = seating.votings {
        for vote in votings {
            if let None = vote.voting_results {
                return Err(PopisError::LogicError(String::from("No result found under voting.")));
            }
        }
        Ok(())
    } else {
        Err(PopisError::LogicError(String::from("No votings found for seating.")))
    }
}

pub async fn insert_seating(provider: &Provider, seating: &Seating) -> Result<()> {
    verify_if_contains_data(seating);
    //add parties somewhere here
    let seating_id = insert_query(provider, r#"
    INSERT INTO seating (date, number) VALUES ($1, $2) RETURNING id;
    "#, &[&seating.date, &seating.number]).await?;
    for voting in seating.votings.as_ref().unwrap().iter() {
        insert_voting(provider, seating_id, voting).await?;
    }
    Ok(())
}

async fn insert_voting(provider: &Provider, seating_id: i64, voting: &Voting) -> Result<()> {
    let voting_id = insert_query(provider, r#"
    INSERT INTO voting (number, seating_id, description) VALUES ($1, $2, $3) RETURNING id;
    "#, &[&voting.number, &seating_id, &voting.description]).await?;
    insert_voting_result(provider, voting_id, voting.voting_results.as_ref().unwrap()).await
}

async fn insert_voting_result(provider: &Provider, voting_id: i64, result: &VotingResult) -> Result<()> {
    for party_vote in result.parties_votes {
        insert_query(provider, r#"
            INSERT INTO vote (voting_id, party_id, result) VALUES ($1, $2, $3);
        "#, &[todo!(), todo!(), ])

    } 
}
id SERIAL PRIMARY KEY,
        voting_id INT NOT NULL,
        party_id INT NOT NULL,
        result INT NOT NULL,