use std::sync::Mutex;

use crate::tafl::game::Game;
use std::collections::HashMap;
use crate::api::game_mgmt::partial_game::PartialGame;

lazy_static! {
    pub static ref GAMESTATE: GameState = GameState {
        full_games: Mutex::new(HashMap::new()),
        part_games: Mutex::new(HashMap::new())
    };
}

pub struct GameState {
    pub full_games: Mutex<HashMap<usize, Game>>,
    pub part_games: Mutex<HashMap<usize, PartialGame>>
}