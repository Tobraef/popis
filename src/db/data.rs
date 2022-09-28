use crate::domain;

pub struct DbVotingHeader {
    pub id: i32,
    pub header: domain::VotingHeader,
}

impl DbVotingHeader {
    pub fn new(id: i32, header: domain::VotingHeader) -> Self { Self { id, header } }
}