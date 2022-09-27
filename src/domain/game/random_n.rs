use std::collections::HashMap;

use crate::{domain::{Vote, PartyVote, VotingHeader}, db::{Provider, self}, popis_error::Result};

use super::data::RandomNGameState;

pub fn begin(limit: u8) -> RandomNGameState {
    RandomNGameState::Asking { limit, asked_so_far: 0, favourites: Default::default() }
}

pub async fn pick_voting(provider: &Provider) -> Result<VotingHeader> {
    db::query::random_voting(provider)
        .await
        .map(|voting| voting.header)
}

pub fn chose(game_state: RandomNGameState, vote: &Vote, parties_votes: &Vec<PartyVote>) -> RandomNGameState {
    match game_state {
        RandomNGameState::Asking { limit, asked_so_far, favourites } => {
            let favourites = resolve_favourites(favourites, vote, parties_votes);
            increment_step_and_return(limit, asked_so_far, favourites)
        },
        RandomNGameState::Result(r) => RandomNGameState::Result(r),
    }
}

fn resolve_favourites(mut favourites: HashMap<String, u8>, vote: &Vote, parties_votes: &Vec<PartyVote>) -> HashMap<String, u8> {
    let voted_alike = parties_votes
        .iter()
        .filter(|pv| vote == &pv.vote)
        .map(|pv| pv.party.name.as_str());
    for alike_party in voted_alike {
        *favourites.entry(alike_party.to_owned()).or_insert(0) += 1;
    }
    favourites
}

fn increment_step_and_return(limit: u8, mut asked_so_far: u8, favourites: HashMap<String, u8>) -> RandomNGameState {
    asked_so_far += 1;
    if limit == asked_so_far {
        let winners = find_winners(favourites);
        RandomNGameState::Result(winners)
    } else {
        RandomNGameState::Asking { limit, asked_so_far, favourites }
    }
}

fn find_winners(favourites: HashMap<String, u8>) -> Vec<String> {
    let max = *favourites
        .values()
        .max()
        .unwrap_or(&0);
    let winners = favourites
        .into_iter()
        .filter(|kv| kv.1 == max)
        .map(|kv| kv.0)
        .collect();
    winners
}

#[cfg(test)]
mod tests {
    use crate::domain::Party;

    use super::*;

    #[test]
    fn single_game() {
        let party_vote = |a: &str, b| PartyVote::new(Party::new(a.to_string()), b); 
        let mut game = begin(3);
        let p1 = vec![
            party_vote("a", Vote::For),
            party_vote("b", Vote::Against),
        ];
        let p2 = vec![
            party_vote("a", Vote::Against),
            party_vote("b", Vote::For),
        ];
        let p3 = vec![
            party_vote("a", Vote::Hold),
            party_vote("b", Vote::Against),
        ];
        
        game = chose(game, &Vote::For, &p1);
        game = chose(game, &Vote::Against, &p2);
        game = chose(game, &Vote::Hold, &p3);

        match game {
            RandomNGameState::Asking { .. } => panic!("Game should have ended so far."),
            RandomNGameState::Result(r) => assert_eq!(r, ["a"]),
        }
    }
}