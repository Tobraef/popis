use std::collections::HashSet;

use crate::popis_error::*;

use super::Seating;

pub fn parties_in_seating(seating: &Seating) -> Result<HashSet<&str>> {
    Ok(seating
        .votings
        .iter()
        .flat_map(|v| {
            v.voting_result
                .parties_votes
                .iter()
                .map(|p| p.party.name.as_str())
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::domain::{SeatingHeader, Voting, VotingHeader, VotingResult, PartyVote, Party, Vote};

    use super::*;

    #[test]
    fn parties_in_seating_test() {
        let seating = Seating::new(SeatingHeader::new(0, Default::default()), vec![
            Voting::new(VotingHeader::new(0, Default::default()), VotingResult::new(vec![
                PartyVote::new(Party::new("a".to_string()), Vote::Against),
                PartyVote::new(Party::new("b".to_string()), Vote::Against),
                PartyVote::new(Party::new("c".to_string()), Vote::Against),
            ])),
            Voting::new(VotingHeader::new(0, Default::default()), VotingResult::new(vec![
                PartyVote::new(Party::new("d".to_string()), Vote::Against),
                PartyVote::new(Party::new("e".to_string()), Vote::Against),
                PartyVote::new(Party::new("f".to_string()), Vote::Against),
            ])),
        ]);
        let parties_found = Vec::from_iter(parties_in_seating(&seating).unwrap());
        assert!(vec!["a", "b", "c", "d", "e", "f"].iter().all(|x| parties_found.contains(x)));
    }
}