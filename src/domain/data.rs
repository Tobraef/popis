use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SeatingHeader {
    pub identifier: i32,
    pub date: DateTime<Utc>,
}

impl SeatingHeader {
    pub fn new(number: i32, date: DateTime<Utc>) -> Self {
        Self { identifier: number, date }
    }
}

#[derive(Debug)]
pub struct Seating {
    pub header: SeatingHeader,
    pub votings: Vec<Voting>,
}

impl Seating {
    pub fn new(header: SeatingHeader, votings: Vec<Voting>) -> Self {
        Self { header, votings }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VotingHeader {
    pub identifier: i32,
    pub description: String,
}

impl VotingHeader {
    pub fn new(number: i32, description: String) -> Self {
        Self {
            identifier: number,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Voting {
    pub header: VotingHeader,
    pub voting_result: VotingResult,
}

impl Voting {
    pub fn new(header: VotingHeader, voting_result: VotingResult) -> Self {
        Self {
            header,
            voting_result,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VotingResult {
    pub parties_votes: Vec<PartyVote>,
}

impl VotingResult {
    pub fn new(parties_votes: Vec<PartyVote>) -> Self {
        Self { parties_votes }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartyVote {
    pub party: Party,
    pub vote: Vote,
}

impl PartyVote {
    pub fn new(party: Party, vote: Vote) -> Self {
        Self { party, vote }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub name: String,
    //maybe members some day
}

impl Party {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Vote {
    For,
    Against,
    Hold,
}

impl Into<Vote> for i32 {
    fn into(self) -> Vote {
        match self {
            0 => Vote::For,
            1 => Vote::Against,
            2 => Vote::Hold,
            _ => panic!(),
        }
    }
}

impl Vote {
    pub fn from_votes(votes_for: u32, votes_against: u32, votes_held: u32) -> Vote {
        let max = votes_for.max(votes_against).max(votes_held);
        match max {
            x if x == votes_for => Vote::For,
            x if x == votes_against => Vote::Against,
            x if x == votes_held => Vote::Hold,
            _ => panic!(),
        }
    }

    pub fn num(&self) -> i32 {
        match self {
            Self::For => 0,
            Self::Against => 1,
            Self::Hold => 2,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_votes_tests() {
        assert!(matches!(Vote::from_votes(1, 0, 2), Vote::Hold));
        assert!(matches!(Vote::from_votes(2, 4, 2), Vote::Against));
        assert!(matches!(Vote::from_votes(5, 4, 3), Vote::For));

        assert!(matches!(Vote::from_votes(1, 1, 2), Vote::Hold));
        assert!(matches!(Vote::from_votes(3, 4, 2), Vote::Against));
        assert!(matches!(Vote::from_votes(2, 2, 2), Vote::For));
    }
}
