use serde::{Deserialize, Serialize};

use super::creature::Creature;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub game_id: String,
    pub phase: GamePhase,
    pub current_turn: Option<String>,
    pub players: Vec<String>,
    pub entities: Vec<Creature>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GamePhase {
    Waiting = 1,
    Creation,
    Battle,
    Finish,
}

#[derive(Deserialize)]
pub struct PollRequest {
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct NameRequest {
    pub name: String,
}