use std::collections::HashMap;
use std::sync::Mutex;

use crate::api::game_mgmt::partial_game::PartialGame;
use crate::tafl::game::Game;

lazy_static! {
    pub static ref GAMESTATE: GameState = GameState {
        full_games: Mutex::new(HashMap::new()),
        part_games: Mutex::new(HashMap::new())
    };
}

pub struct GameState {
    pub full_games: Mutex<HashMap<usize, Game>>,
    pub part_games: Mutex<HashMap<usize, PartialGame>>,
}