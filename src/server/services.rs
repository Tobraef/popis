use actix_web::{web::{Json, Data}, get};
use tokio::sync::Mutex;

use crate::{popis_error::{PopisError, Result}, db::Provider, domain::{random_n_game::{data::{GameResponse, PlayRequest}, orchestrator::Orchestrator}}};

type OrchestratorType<'a> = Data<Mutex<Orchestrator<'a>>>;

#[get("/start_random_n/{n}")]
pub async fn start_random_n(n: String, orchestrator: OrchestratorType<'_>) -> Result<Json<GameResponse>> {
    let mut orchestrator = orchestrator.lock().await;
    orchestrator.start_new(n.parse().map_err(|_e| PopisError::GameplayError(format!("Couldn't parse n: {n} into a number")))?)
        .await
        .map(|resp| Json(resp))
}

#[get("/play_random_n/{n}/{id}")]
pub async fn play_random_n(n: String, id: String, orchestrator: OrchestratorType<'_>) -> Result<Json<GameResponse>> {
    //make post and PlayRequest from json
    let picked = n.parse().map_err(|_e| PopisError::GameplayError(format!("Couldn't parse n: {n} into a number")))?;
    let id = id.parse().map_err(|_e| PopisError::GameplayError(format!("Couldn't parse id: {id} into a number")))?;
    let mut orchestrator = orchestrator.lock().await;
    orchestrator.player_chose(PlayRequest::new(picked, id))
        .await
        .map(|resp| Json(resp))
}

pub async fn serve(provider: &'static Provider) -> Result<()> {
    use std::net::*;
    let games_orchestrator = Data::new(Mutex::new(Orchestrator::new(provider)));
    let provider = Data::new(provider);
    actix_web::HttpServer::new(move || actix_web::App::new()
        .app_data(provider.clone())
        .app_data(games_orchestrator.clone())
        .service(start_random_n)
        .service(play_random_n)
    )
        .bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8123))
        .map_err(|e| PopisError::ServerStart(e.to_string()))?
        .run()
        .await
        .map_err(|e| PopisError::ServerStart(e.to_string()))
}