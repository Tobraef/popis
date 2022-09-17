use crate::domain::Vote;

mod domain;
mod fetching;
mod popis_error;

fn logger() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[derive(Eq, Default, Hash)]
struct Group {
    parties: Vec<String>,
}

impl Group {
    fn new(parties: Vec<String>) -> Self { Self { parties } }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.parties.len() == other.parties.len() &&
        self.parties.iter().all(|p| other.parties.contains(p))
    }
}

#[tokio::main]
async fn main() {
    logger();
    let mut groups = std::collections::HashMap::new();
    let seatings = fetching::fetch_seatings(9).await.unwrap();
    for seating in seatings.seatings.into_iter().take(30) {
        match fetching::fetch_votings(seating.link).await {
            Ok(votings) => for voting in votings {
                log::info!("{}", voting.description);
                match fetching::fetch_voting_results(voting.link).await {
                    Ok(result) => {
                        let for_c = result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::For)).count();
                        let aga_c = result.parties_votes.iter().filter(|v| matches!(v.vote, Vote::Against)).count();
                        match for_c > aga_c {
                            true => {
                                *groups
                                .entry(Group::new(result.parties_votes.into_iter().filter(|v| !matches!(v.vote, Vote::Against)).map(|x| x.party.name.clone()).collect()))
                                .or_insert(0) += 1;
                            },
                            false => {
                                *groups
                                .entry(Group::new(result.parties_votes.into_iter().filter(|v| !matches!(v.vote, Vote::For)).map(|x| x.party.name.clone()).collect()))
                                .or_insert(0) += 1;
                            },
                        }
                    },
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("{}", e);
            },
        }
    }
    for g in groups.iter().filter(|x| x.1 >= &10) {
        log::info!("{:?} {}", g.0.parties, g.1);
    }
}
