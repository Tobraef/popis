use futures::future::ok;

use crate::{db::{Provider, self}, popis_error::{Result, PopisError}};

use super::{GameState, engine::{begin, player_chose}, data::{PlayRequest, GameResponse}};

pub struct Orchestrator<'a> {
    game_state: GameState,
    provider: &'a Provider,
}

impl<'a> Orchestrator<'a> {
    pub fn new(provider: &'a Provider) -> Self { Self { provider, game_state: Default::default() } }

    async fn next_voting(&self) -> Result<GameResponse> {
        db::query::random_voting_header(self.provider)
            .await
            .map(|voting| GameResponse::Play { voting_description: voting.header.description, voting_id: voting.id })
    }

    pub async fn start_new(&mut self, limit: u8) -> Result<GameResponse> {
        self.game_state = begin(limit);
        self.next_voting().await
    }

    pub async fn player_chose(&mut self, req: PlayRequest) -> Result<GameResponse> {
        let voting_result = db::query::voting_results(self.provider, req.voting_id)
            .await?;
        match player_chose(&mut self.game_state, &req.vote.into(), &voting_result) {
            Some(favourite_parties) => ok::<GameResponse, PopisError>(GameResponse::Result { favourite_parties }).await,
            None => self.next_voting().await,
        }
    }
}
