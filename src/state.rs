use std::sync::Mutex;

use crate::tafl::board::BoardConfiguration;

lazy_static! {
    pub static ref state: TafliState = TafliState {
        board: Mutex::new(BoardConfiguration::new())
    };
}

pub struct TafliState {
    pub board: Mutex<BoardConfiguration>
}