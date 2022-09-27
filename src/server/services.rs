use actix_web::{web::{Json, Data}, get};
use tokio::sync::Mutex;

use crate::{popis_error::{PopisError, Result}, db::{self, Provider}, domain::{Voting, game::{RandomNGameState, random_n}, Vote}};

#[get("/random_voting")]
pub async fn random_voting(provider: Data<&Provider>) -> Result<Json<Voting>> {
    let voting = db::query::random_voting(provider.as_ref()).await?;
    Ok(Json(voting))
}

#[get("/start_random_n/{n}")]
pub async fn start_random_n(n: String, game_state: Data<Mutex<Option<RandomNGameState>>>, provider: Data<&Provider>) -> Result<Json<String>> {
    let mut game_state = game_state.lock().await;
    game_state.insert(random_n::begin(n.parse().unwrap()));
    let voting = random_n::pick_voting(provider.as_ref()).await?;
    Ok(Json(voting.description))
}

#[get("/play_random_n/{n}")]
pub async fn play_random_n(n: String, game_state: Data<Mutex<Option<RandomNGameState>>>, provider: Data<&Provider>) -> Result<Json<String>> {
    let picked: Vote = n.parse::<i32>().unwrap().into();
    let mut game_state = game_state.lock().await;
    game_state = random_n::chose(game_state, &vote, parties_votes)
}

pub async fn serve(provider: &'static Provider) -> Result<()> {
    use std::net::*;
    let provider = Data::new(provider);
    let random_n_game_state = Data::new(Mutex::new(Option::<RandomNGameState>::None));
    actix_web::HttpServer::new(move || actix_web::App::new()
        .app_data(provider.clone())
        .app_data(random_n_game_state.clone())
        .service(random_voting)
        .service(start_random_n)
        .service(play_random_n)
    )
        .bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8123))
        .map_err(|e| PopisError::ServerStart(e.to_string()))?
        .run()
        .await
        .map_err(|e| PopisError::ServerStart(e.to_string()))
}