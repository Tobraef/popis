use chrono::{Utc, DateTime};

#[derive(Debug)]
pub struct SeatingList {
    pub seatings: Vec<Seating>,
}

impl SeatingList {
    pub fn new(seatings: Vec<Seating>) -> Self {
        Self { seatings }
    }
}

#[derive(Debug)]
pub struct Url(pub String);

impl Url {
    pub fn try_new(s: String) -> Option<Url> {
        reqwest::Url::parse(&s).ok().map(|_| Url(s))
    }
}

#[derive(Debug)]
pub struct Seating {
    pub link: Url,
    pub date: DateTime<Utc>,
    pub number: u32,
    pub votings: Option<Vec<Voting>>,
}

impl Seating {
    pub fn new(link: Url, date: DateTime<Utc>, number: u32) -> Self {
        Self {
            link,
            date,
            number,
            votings: None,
        }
    }
}

#[derive(Debug)]
pub struct Voting {
    pub link: Url,
    pub number: u32,
    pub description: String,
    pub voting_results: Option<VotingResult>,
}

impl Voting {
    pub fn new(link: Url, number: u32, description: String) -> Self {
        Self {
            link,
            number,
            description,
            voting_results: None,
        }
    }
}

#[derive(Debug)]
pub struct VotingResult {
    pub parties_votes: Vec<PartyVote>,
}

impl VotingResult {
    pub fn new(parties_votes: Vec<PartyVote>) -> Self {
        Self { parties_votes }
    }
}

#[derive(Debug)]
pub struct PartyVote {
    pub party: Party,
    pub vote: Vote,
}

impl PartyVote {
    pub fn new(party: Party, vote: Vote) -> Self {
        Self { party, vote }
    }
}

#[derive(Debug)]
pub struct Party {
    pub name: String,
    //maybe members some day
}

impl Party {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub enum Vote {
    For,
    Against,
    Hold,
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
}

impl From<i64> for Vote {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::For,
            1 => Self::Against,
            2 => Self::Hold,
            _ => panic!(),
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
