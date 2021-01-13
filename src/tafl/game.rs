use crate::tafl::board::{BoardConfiguration, Player};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub gameid: usize,
    pub player_white: PlayerInfo,
    pub player_black: PlayerInfo,
    pub board: BoardConfiguration,
    pub status: GameStatus,
    pub prev_move_info: PreviousMoveInfo,
    pub time_control: (usize, usize),
}

impl Game {
    pub fn new(gameid: usize, player_white: PlayerInfo, player_black: PlayerInfo, time_control: (usize, usize)) -> Self {
        Game {
            gameid,
            player_white,
            player_black,
            board: BoardConfiguration::new(),
            status: GameStatus::Playing,
            prev_move_info: PreviousMoveInfo::new(),
            time_control,
        }
    }

    pub fn from_file(str: &'static str) -> Self {
        Game {
            gameid: 0,
            player_white: PlayerInfo { userid: 0, name: String::from("white") },
            player_black: PlayerInfo { userid: 1, name: String::from("black") },
            board: BoardConfiguration::from_file(str),
            status: GameStatus::Playing,
            prev_move_info: PreviousMoveInfo::new(),
            time_control: (10, 10),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub userid: usize,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GameStatus {
    Playing,
    Won(Player, String),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PreviousMoveInfo {
    pub last_move: Option<((usize, usize), (usize, usize))>,
    pub captures: Vec<(usize, usize)>,
}

impl PreviousMoveInfo {
    pub(crate) fn new() -> Self {
        PreviousMoveInfo {
            last_move: None,
            captures: Vec::new(),
        }
    }
}