use crate::tafl::board::{BoardConfiguration, Player};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: BoardConfiguration,
    pub status: GameStatus
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: BoardConfiguration::new(),
            status: GameStatus::Playing
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GameStatus {
    Playing,
    Won(Player, String)
}