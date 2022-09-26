use actix_web::{web::{Json, Data}, get};

use crate::{popis_error::{PopisError, Result}, db::{self, Provider}, domain::Voting};

#[get("/random_voting")]
pub async fn random_voting(provider: Data<&Provider>) -> Result<Json<Voting>> {
    let voting = db::query::random_voting(provider.as_ref()).await?;
    Ok(Json(voting))
}

pub async fn serve(provider: &'static Provider) -> Result<()> {
    use std::net::*;
    let provider = Data::new(provider);
    actix_web::HttpServer::new(move || actix_web::App::new()
        .app_data(provider.clone())
        .service(random_voting)
    )
        .bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8123))
        .map_err(|e| PopisError::ServerStart(e.to_string()))?
        .run()
        .await
        .map_err(|e| PopisError::ServerStart(e.to_string()))
}