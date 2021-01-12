use std::sync::Mutex;

use crate::tafl::game::Game;

lazy_static! {
    pub static ref state: TafliState = TafliState {
        game: Mutex::new(Game::new())
    };
}

pub struct TafliState {
    pub game: Mutex<Game>
}