mod domain;
mod fetching;
mod popis_error;

fn logger() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() {
    logger();
    let seatings = fetching::fetch_seatings(9).await.unwrap();
    for seating in seatings.seatings {
        log::info!("{} {} {}", seating.number, seating.date.to_string(), seating.link.0);
        let votings = fetching::fetch_votings(seating.link).await.unwrap();
        for voting in votings {
            log::info!("{} {} {}", voting.description, voting.link.0, voting.number);
        }
    }
}
