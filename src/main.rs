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
    for seating in seatings.seatings.iter() {
        println!("{} {} {}", seating.number, seating.date.to_string(), seating.link.0);
    }
}
