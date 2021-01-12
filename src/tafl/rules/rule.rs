use crate::tafl::game::Game;

pub trait Rule {
    fn make_move(&self, game: &mut Game, from: (usize, usize), to: (usize, usize));
}