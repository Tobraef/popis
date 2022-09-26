use crate::domain::{SeatingHeader, VotingHeader};

#[derive(Debug)]
pub struct Url(pub String);

impl Url {
    pub fn try_new(s: String) -> Option<Url> {
        reqwest::Url::parse(&s).ok().map(|_| Url(s))
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct LoadableSeatingHeader {
    pub header: SeatingHeader,
    pub(super) votings_url: Url,
}

impl LoadableSeatingHeader {
    pub fn new(header: SeatingHeader, url_to_load: Url) -> Self {
        Self {
            header,
            votings_url: url_to_load,
        }
    }
}

#[derive(Debug)]
pub(super) struct LoadableVoting {
    pub voting: VotingHeader,
    pub voting_result_url: Url,
}

impl LoadableVoting {
    pub(super) fn new(voting: VotingHeader, voting_result_url: Url) -> Self {
        Self {
            voting,
            voting_result_url,
        }
    }
}
