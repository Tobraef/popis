mod tools;
mod document_parsing;

use tools::*;

use crate::{domain::{Url, Voting, SeatingList}, popis_error::Result};

pub async fn fetch_votings(link: Url) -> Result<Vec<Voting>> {
    let document = fetch_document(&link.0)
        .await?;
    document_parsing::parse_votings(document)    
}

pub async fn fetch_seatings(cadence: u32) -> Result<SeatingList> {
    let url = seatings_url(cadence);
    let document = fetch_document(url)
        .await?;
    document_parsing::parse_seatings(document)
}