use crate::popis_error::*;

use super::Seating;

pub fn parties_in_seating(seating: &Seating) -> Result<Vec<&str>> {
    Ok(seating
        .votings
        .iter()
        .map(|v| {
            v.voting_result
                .parties_votes
                .iter()
                .map(|p| p.party.name.as_str())
        })
        .find(|p| p.clone().next().is_some())
        .ok_or_else(|| PopisError::LogicError("Seating doesn't contain any parties in it.".into()))?
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
        let parties_found = parties_in_seating(&seating).unwrap();
        assert_eq!(vec!["a".to_string(), "b".to_string(), "c".to_string()], parties_found);
    }
}