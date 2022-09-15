use chrono::{Date, Utc};

pub struct SeatingList {
    pub seatings: Vec<Seating>,
}

impl SeatingList {
    pub fn new(seatings: Vec<Seating>) -> Self { Self { seatings } }
}

pub struct Url(pub String);

impl Url {
    pub fn try_new(s: String) -> Option<Url> {
        reqwest::Url::parse(&s)
            .ok()
            .map(|_| Url(s))
    }
}

pub struct Seating {
    pub link: Url,
    pub date: Date<Utc>,
    pub number: u32,
    pub votings: Option<Vec<Voting>>,
}

impl Seating {
    pub fn new(link: Url, date: Date<Utc>, number: u32) -> Self { Self { link, date, number, votings: None } }
}

pub struct Voting {
    pub link: Url,
    pub number: u32,
    pub description: String,
    pub voting_results: Option<VotingResult>,
}

impl Voting {
    pub fn new(link: Url, number: u32, description: String) -> Self { Self { link, number, description, voting_results: None } }
}

pub struct VotingResult {
    pub parties_votes: Vec<PartyVote>,
}

impl VotingResult {
    pub fn new(parties_votes: Vec<PartyVote>) -> Self { Self { parties_votes } }
}

pub struct PartyVote {
    pub party: Party,
    pub vote: Vote, 
}

impl PartyVote {
    pub fn new(party: Party, vote: Vote) -> Self { Self { party, vote } }
}

pub struct Party {
    pub name: String,
    //maybe members some day
}

impl Party {
    pub fn new(name: String) -> Self { Self { name } }
}

pub enum Vote {
    For,
    Against,
    Hold,
}