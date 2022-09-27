use std::collections::{HashMap, HashSet};

use log::info;
use postgres::types::ToSql;

use super::provider::Provider;
use crate::{
    domain::{parties_in_seating, Seating, Voting, VotingResult},
    popis_error::{PopisError, Result},
};

async fn insert_query(
    provider: &Provider,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<impl Iterator<Item = i32>> {
    let db = &provider.client;
    db.query(query, params)
        .await
        .map_err(|e| {
            PopisError::DbCommunicationError(format!(
                "Couldn't insert data with params:{params:?} and query: {query} into db: {e}"
            ))
        })
        .map(|rows| {
            info!("Inserted succesfully params:{params:?} with query: {query}");
            rows.into_iter().map(|r| r.get::<usize, i32>(0))
        })
}
pub async fn insert_seating(provider: &Provider, seating: &Seating) -> Result<()> {
    let mut parties = insert_and_fetch_parties(provider, seating).await?;
    let seating_id = insert_query(
        provider,
        r#"
    INSERT INTO seating (date, identifier) VALUES ($1, $2) RETURNING id;
    "#,
        &[&seating.header.date, &seating.header.identifier],
    )
    .await?
    .next()
    .unwrap();
    for voting in seating.votings.iter() {
        insert_voting(provider, seating_id, voting, &mut parties).await?;
    }
    Ok(())
}

async fn insert_and_fetch_parties(
    provider: &Provider,
    seating: &Seating,
) -> Result<HashMap<String, i32>> {
    let mut parties = parties_in_seating(seating)?;
    let mut parties_in_db: HashMap<_, _> = super::query::parties(provider)
        .await?
        .collect();
    info!("Parties in seating {parties:?}, parties in db {parties_in_db:?}");
    parties.retain(|&p| !parties_in_db.contains_key(p));
    if !parties.is_empty() {
        insert_missing_parties(provider, parties, &mut parties_in_db).await?;
    }
    Ok(parties_in_db)

}

async fn insert_missing_parties(provider: &Provider, parties: HashSet<&str>, parties_in_db: &mut HashMap<String, i32>) -> Result<()> {
    let values_list = (1..=parties.len())
        .map(|i| format!("(${})", i))
        .collect::<Vec<_>>()
        .join(",");
    let ids = insert_query(
        provider,
        &format!("INSERT INTO party (name) VALUES {} RETURNING id;", values_list),
        &parties
            .iter()
            .map(|x| x as &(dyn ToSql + Sync))
            .collect::<Vec<_>>(),
    )
    .await?;
    parties
        .into_iter()
        .zip(ids)
        .for_each(|(p,id)| { let _ = parties_in_db.insert(p.to_owned(), id); });
    Ok(())
}

async fn insert_voting(
    provider: &Provider,
    seating_id: i32,
    voting: &Voting,
    parties: &mut HashMap<String, i32>,
) -> Result<()> {
    let voting_id = insert_query(
        provider,
        r#"
    INSERT INTO voting (identifier, seating_id, description) VALUES ($1, $2, $3) RETURNING id;
    "#,
        &[
            &voting.header.identifier,
            &seating_id,
            &voting.header.description,
        ],
    )
    .await?
    .next()
    .unwrap();
    insert_voting_result(provider, voting_id, parties, &voting.voting_result).await
}

async fn insert_voting_result(
    provider: &Provider,
    voting_id: i32,
    parties: &mut HashMap<String, i32>,
    result: &VotingResult,
) -> Result<()> {
    let values = (0..result.parties_votes.len())
        .map(|mut i| {
            i *= 3;
            format!("(${},${},${})", i + 1, i + 2, i + 3)
        })
        .collect::<Vec<_>>()
        .join(",");
    let mut query = r#"INSERT INTO vote (voting_id, party_id, result) VALUES "#.to_string();
    query.push_str(&values);
    query.push(';');
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(result.parties_votes.len() * 3);
    let voting_nums: Vec<_> = result.parties_votes.iter().map(|x| x.vote.num()).collect();
    for (vote, num) in result.parties_votes.iter().zip(voting_nums.iter()) {
        params.push(&voting_id);
        params.push(parties.get(&vote.party.name)
            .ok_or_else(|| PopisError::LogicError("All parties should be in cache already."))?);
        params.push(num);
    }
    insert_query(provider, &query, &params).await.map(|_| ())
}
