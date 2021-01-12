use crate::tafl::board::BoardConfiguration;
use crate::tafl::board::FieldState::{Empty, WhiteKing};
use crate::tafl::game::Game;
use crate::tafl::game_move::MakeMoveError::{IllegalMove, WrongPlayer};
use crate::tafl::rules::capture_rule::CaptureRule;
use crate::tafl::rules::rule::Rule;
use crate::tafl::rules::shield_wall_rule::ShieldWallRule;
use crate::tafl::rules::win_white_corner::WinWhiteCorner;

impl Game {
    pub fn rules() -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(CaptureRule {}),
            Box::new(ShieldWallRule {}),
            Box::new(WinWhiteCorner {})
        ]
    }

    /// Returns a Vec of all legal moves for the piece in the given position
    pub fn legal_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        //Empty tiles cannot move
        if self.board[from] == Empty { return Vec::new(); }

        //For each wind direction, iterate
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|dir| {
            //How many times to move in this direction from the start
            (1..)
                //Map the count to the position in this direction
                .map(move |count| (from.0 as isize + count * dir.0, from.1 as isize + count * dir.1))

                //Take while the squares are in the board
                .take_while(|pos| { pos.0 >= 0 && pos.1 >= 0 && pos.0 < 11 && pos.1 < 11 })
                //Take while the squares are empty
                .take_while(|pos| { self.board[*pos] == Empty })
                //Map to (usize, usize)
                .map(|pos| (pos.0 as usize, pos.1 as usize))
                //If this is not the king, we are not allowed to move on the special squares
                .take_while(|pos| {
                    self.board[from] == WhiteKing || !BoardConfiguration::special_squares().contains(pos)
                })
        }).flatten().collect()
    }

    /// Moves the piece in the `from` position to the `to` position
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), MakeMoveError> {
        //Check if move is legal
        if !self.legal_moves(from).contains(&to) { return Err(IllegalMove); }
        //Check if move is made by the right player
        if self.board[from].player().unwrap() != self.board.turn { return Err(WrongPlayer); }

        //Update last move info
        self.prev_move_info.last_move = Some((from, to));
        self.prev_move_info.captures.clear();

        //Move piece
        self.board[to] = self.board[from];
        self.board[from] = Empty;

        //Apply rules
        Self::rules().iter().for_each(|rule| {
            rule.make_move(self, from, to);
        });

        //Switch turn
        self.board.turn = self.board.turn.other();

        return Ok(());
    }

    pub fn capture(&mut self, pos: (usize, usize)) {
        if self.board[pos] == Empty { panic!("Illegal capture!") }
        self.board[pos] = Empty;
        self.prev_move_info.captures.push(pos);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MakeMoveError {
    IllegalMove,
    WrongPlayer,
}

#[cfg(test)]
mod test {
    use crate::tafl::board::FieldState::{BlackPiece, Empty};
    use crate::tafl::game_move::MakeMoveError::IllegalMove;

    use super::*;

    #[test]
    fn test_legal_moves() {
        let game = Game::new();
        assert_eq!(vec![(0, 0); 0], game.legal_moves((5, 0)));
        assert_eq!(vec![(2, 0), (1, 0), (3, 1), (3, 2), (3, 3), (3, 4)], game.legal_moves((3, 0)));
        assert_eq!(vec![(6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (4, 1), (3, 1), (2, 1), (1, 1), (0, 1), (5, 2)], game.legal_moves((5, 1)));
        assert_eq!(vec![(0, 0); 0], game.legal_moves((5, 2)));
        assert_eq!(vec![(6, 3), (7, 3), (8, 3), (9, 3), (4, 3), (3, 3), (2, 3), (1, 3), (5, 2)], game.legal_moves((5, 3)));
    }

    #[test]
    fn test_make_move() {
        let mut game = Game::new();
        assert_eq!(Err(IllegalMove), game.make_move((0, 0), (1, 0)));
        assert_eq!(game.board.fields[3][0], BlackPiece);
        assert_eq!(game.board.fields[2][0], Empty);
        assert_eq!(Ok(()), game.make_move((3, 0), (2, 0)));
        assert_eq!(game.board.fields[3][0], Empty);
        assert_eq!(game.board.fields[2][0], BlackPiece);
    }
}