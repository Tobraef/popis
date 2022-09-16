use crate::domain::Vote;

mod domain;
mod fetching;
mod popis_error;

fn logger() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Warn)
        .init();
}

#[tokio::main]
async fn main() {
    logger();
    let seatings = fetching::fetch_seatings(9).await.unwrap();
    for seating in seatings.seatings {
        log::info!("{} {} {}", seating.number, seating.date.to_string(), seating.link.0);
        match fetching::fetch_votings(seating.link).await {
            Ok(votings) => for voting in votings {
                log::info!("{} {} {}", voting.description, voting.link.0, voting.number);
                match fetching::fetch_voting_results(voting.link).await {
                    Ok(result) => {
                        let for_c = result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::For)).count();
                        let aga_c = result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::Against)).count();
                        match for_c > aga_c {
                            true => for p in result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::Against)) {
                                log::warn!("{}", p.party.name);
                            },
                            false => for p in result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::For)) {
                                log::warn!("{}", p.party.name);
                            },
                        }
                    },
                    Err(e) => {
                        log::error!("{}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                log::error!("{}", e);
                return;
            },
        }
    }
}
