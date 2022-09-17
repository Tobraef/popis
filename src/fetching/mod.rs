mod tools;
mod document_parsing;

use tools::*;

use crate::{domain::{Url, Voting, SeatingList, VotingResult}, popis_error::Result};

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

pub async fn fetch_voting_results(link: Url) -> Result<VotingResult> {
    let document = fetch_document(link.0)
        .await?;
    document_parsing::parse_voting_result(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetching_all() {
        let seatings = fetch_seatings(9).await.unwrap();
        let mut valid_seatings = 0;
        let mut total_seatings = 0;
        let mut valid_votings = 0;
        let mut total_votings = 0;
        for seating in seatings.seatings.into_iter().take(10) {
            total_seatings += 1;
            let link = seating.link;
            if let Ok(votings) = fetch_votings(link).await {
                valid_seatings += 1;
                for voting in votings.into_iter().take(20) {
                    if let Ok(_) = fetch_voting_results(voting.link).await {
                        valid_votings += 1;
                    }
                    total_votings += 1;
                }
            }
        }
        assert!(total_seatings * 9 / 10 < valid_seatings, "Total seatings: {}, valid ones: {}", total_seatings, valid_seatings);
        assert!(total_votings * 9 / 10 < valid_votings, "Total votings: {}, valid ones: {}", total_votings, valid_votings);
    }
}