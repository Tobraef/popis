mod data;
mod document_parsing;
mod tools;

use futures::{stream::FuturesUnordered, StreamExt};
use tools::*;

use crate::{
    domain::{Seating, Voting},
    popis_error::Result,
};

use self::data::LoadableSeatingHeader;

pub async fn fetch_seating_headers(cadence: u32) -> Result<Vec<LoadableSeatingHeader>> {
    let url = seatings_url(cadence);
    let document = fetch_document(url.as_ref()).await?;
    document_parsing::parse_seating_list(&document)
}

pub async fn load_seating(loadable_seating_header: LoadableSeatingHeader) -> Result<Seating> {
    let votings_document = fetch_document(loadable_seating_header.votings_url.as_ref()).await?;
    let loadable_votings = document_parsing::parse_votings(&votings_document)?;
    let mut votings = Vec::with_capacity(loadable_votings.len());
    let mut tasks =
        FuturesUnordered::from_iter(loadable_votings.into_iter().map(|loadable| async move {
            fetch_document(loadable.voting_result_url.as_ref())
                .await
                .and_then(|result_document| document_parsing::parse_voting_result(&result_document))
                .map(|voting_result| Voting::new(loadable.voting, voting_result))
                .ok()
        }));
    while let Some(Some(voting)) = tasks.next().await {
        votings.push(voting);
    }
    Ok(Seating::new(loadable_seating_header.header, votings))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetching_all() {
        let seatings = fetch_seating_headers(9).await.unwrap();
        let mut ok = 0;
        for seating in seatings.into_iter().take(5) {
            if let Ok(_seating) = load_seating(seating).await {
                ok += 1;
                if ok == 3 {
                    break;
                }
            }
        }
        assert_eq!(ok, 3);
    }
}
