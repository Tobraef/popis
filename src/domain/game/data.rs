use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub enum RandomNGameState {
    Asking {
        limit: u8,
        asked_so_far: u8,
        favourites: HashMap<String, u8>,
    },
    Result(Vec<String>),
}

#[derive(Serialize, Deserialize)]
pub struct VotingResponse {
    pub description: String,
    pub id: i32,
}