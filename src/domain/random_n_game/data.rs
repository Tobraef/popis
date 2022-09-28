use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct  GameState {
    pub(super) limit: u8,
    pub(super) asked_so_far: u8,
    pub(super) favourites: HashMap<String, u8>,
}

impl GameState {
    pub fn new(limit: u8) -> Self { Self { limit, ..Default::default() } }
}

#[derive(Serialize, Deserialize)]
pub enum GameResponse {
    Play {
        voting_description: String,
        voting_id: i32,
    },
    Result {
        favourite_parties: Vec<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct PlayRequest {
    pub vote: i32,
    pub voting_id: i32,
}

impl PlayRequest {
    pub fn new(vote: i32, voting_id: i32) -> Self { Self { vote, voting_id } }
}