use db::Provider;
use fetching::load_seating;
use log::error;
use popis_error::*;

mod db;
mod domain;
mod fetching;
mod popis_error;
mod server;

fn logger() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

async fn fill_db(provider: &Provider) -> Result<()> {
    let seatings = fetching::fetch_seating_headers(9).await?;
    for seating in seatings {
        if !db::query::contains_seating(provider, &seating.header).await? {
            let seating = match load_seating(seating).await {
                Ok(v) => v,
                Err(e) => {
                    error!("Couldn't load a seating: {}", e);
                    continue;
                }
            };
            db::command::insert_seating(provider, &seating).await?;
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    static mut DB_PROVIDER: Option<Provider> = None;
    logger();
    //SAFETY required by postgres to be non cloneable and by actix to be static
    let provider = unsafe {
        DB_PROVIDER = Some(db::Provider::new().await?);
        DB_PROVIDER.as_ref().unwrap()
    };
    fill_db(&provider).await?;
    server::serve(&provider).await?;
    Ok(())
}
