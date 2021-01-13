use std::sync::Mutex;

use crate::tafl::game::Game;
use std::collections::HashMap;

lazy_static! {
    pub static ref GAMES: GameState = GameState {
        games: Mutex::new(HashMap::new())
    };
}

pub struct GameState {
    pub games: Mutex<HashMap<usize, Game>>
}