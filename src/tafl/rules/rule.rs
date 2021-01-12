use crate::tafl::board::BoardConfiguration;

pub trait Rule {
    fn make_move(&self, board: &mut BoardConfiguration, from: (usize, usize), to: (usize, usize));
}