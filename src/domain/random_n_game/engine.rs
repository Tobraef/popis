use std::collections::HashMap;

use crate::{domain::{Vote, PartyVote}};

use super::data::GameState;

pub fn begin(limit: u8) -> GameState {
    GameState::new(limit)
}

pub fn player_chose(game_state: &mut GameState, vote: &Vote, parties_votes: &Vec<PartyVote>) -> Option<Vec<String>> {
    if game_state.limit > game_state.asked_so_far {
        resolve_favourites(&mut game_state.favourites, vote, parties_votes);
        game_state.asked_so_far += 1;
        None
    } else {
        Some(find_winners(&game_state.favourites))
    }
}

fn resolve_favourites(favourites: &mut HashMap<String, u8>, vote: &Vote, parties_votes: &Vec<PartyVote>) {
    let voted_alike = parties_votes
        .iter()
        .filter(|pv| vote == &pv.vote)
        .map(|pv| pv.party.name.as_str());
    for alike_party in voted_alike {
        *favourites.entry(alike_party.to_owned()).or_insert(0) += 1;
    }
}

fn find_winners(favourites: &HashMap<String, u8>) -> Vec<String> {
    let max = favourites
        .values()
        .max()
        .unwrap_or(&0);
    favourites
        .into_iter()
        .filter(|kv| kv.1 == max)
        .map(|kv| kv.0.to_owned())
        .collect()
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
        
        assert!(player_chose(&mut game, &Vote::For, &p1).is_none());
        assert!(player_chose(&mut game, &Vote::Against, &p2).is_none());
        let result = player_chose(&mut game, &Vote::Hold, &p3).unwrap();

        assert_eq!(result, ["a"]);
    }
}