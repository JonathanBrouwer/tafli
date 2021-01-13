use crate::prev_move_info::PreviousMoveInfo;
use crate::tafl::board::{BoardConfiguration, Player};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub gameid: usize,
    pub player_white: PlayerInfo,
    pub player_black: PlayerInfo,
    pub board: BoardConfiguration,
    pub status: GameStatus,
    pub prev_move_info: PreviousMoveInfo,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub userid: usize,
    pub name: String
}

impl Game {
    pub fn new(gameid: usize, player_white: PlayerInfo, player_black: PlayerInfo) -> Self {
        Game {
            gameid, player_white, player_black,
            board: BoardConfiguration::new(),
            status: GameStatus::Playing,
            prev_move_info: PreviousMoveInfo::new(),
        }
    }

    pub fn from_file(str: &'static str) -> Self {
        Game {
            gameid: 0,
            player_white: PlayerInfo{userid: 0, name: String::from("white")},
            player_black: PlayerInfo{userid: 1, name: String::from("black")},
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