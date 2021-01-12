use crate::prev_move_info::PreviousMoveInfo;
use crate::tafl::board::{BoardConfiguration, Player};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: BoardConfiguration,
    pub status: GameStatus,
    pub prev_move_info: PreviousMoveInfo,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: BoardConfiguration::new(),
            status: GameStatus::Playing,
            prev_move_info: PreviousMoveInfo::new(),
        }
    }

    pub fn from_file(str: &'static str) -> Self {
        Game {
            board: BoardConfiguration::from_file(str),
            status: GameStatus::Playing,
            prev_move_info: PreviousMoveInfo::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GameStatus {
    Playing,
    Won(Player, String),
}