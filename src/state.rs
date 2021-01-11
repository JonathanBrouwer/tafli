use std::sync::Mutex;
use crate::tafl::board::BoardConfiguration;

pub struct TafliState {
    pub board: Mutex<BoardConfiguration>
}