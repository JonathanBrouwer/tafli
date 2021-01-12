use std::convert::TryInto;

use crate::tafl::board::FieldState::{BlackPiece, Empty, WhiteKing, WhitePiece};
use crate::tafl::board::MakeMoveError::{IllegalMove, WrongPlayer};
use crate::tafl::board::Player::{Black, White};
use crate::tafl::rules::capture_rule::CaptureRule;
use crate::tafl::rules::shield_wall_rule::ShieldWallRule;
use crate::tafl::rules::rule::Rule;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct BoardConfiguration {
    pub fields: [[FieldState; 11]; 11],
    pub turn: Player,
}

impl BoardConfiguration {
    /// Creates a new board in the starting configuration
    pub fn new() -> Self {
        Self::from_file(include_str!("../assets/tafl_board.txt"))
    }

    pub fn from_file(str: &'static str) -> Self {
        let lines: Vec<_> = str.lines().collect();
        let mut fields = [[Empty; 11]; 11];
        for x in 0..11 {
            for y in 0..11 {
                fields[x][y] = match lines[y].chars().nth(x).unwrap() {
                    'W' => WhitePiece,
                    'K' => WhiteKing,
                    'B' => BlackPiece,
                    'x' => Empty,
                    _ => unreachable!()
                };
            }
        }

        BoardConfiguration { fields, turn: Player::Black, }
    }

    pub fn rules() -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(CaptureRule {}),
            Box::new(ShieldWallRule {})
        ]
    }

    pub fn special_squares() -> [(usize, usize); 5] {
        return [(0, 0), (0, 10), (10, 0), (10, 10), (5, 5)];
    }

    pub fn can_capture_with(&self, to_capture: (usize, usize), capture_with: (usize, usize)) -> bool {
        match (self[to_capture].player(),
               self[capture_with].player(),
               Self::special_squares().contains(&capture_with)) {
            //Capture using friendly piece
            (Some(p1), Some(p2), _) if p1 != p2 => true,
            //Capture using special square, which does not contain their own piece
            (Some(p1), p2, true) if Some(p1) != p2 => true,
            //No other way to capture
            _ => false
        }
    }

    /// Returns a Vec of all legal moves for the piece in the given position
    pub fn legal_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        //Empty tiles cannot move
        if self[from] == Empty { return Vec::new(); }

        //For each wind direction, iterate
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|dir| {
            //How many times to move in this direction from the start
            (1..)
                //Map the count to the position in this direction
                .map(move |count| (from.0 as isize + count * dir.0, from.1 as isize + count * dir.1))

                //Take while the squares are in the board
                .take_while(|pos| { pos.0 >= 0 && pos.1 >= 0 && pos.0 < 11 && pos.1 < 11 })
                //Take while the squares are empty
                .take_while(|pos| { self[*pos] == Empty })
                //Map to (usize, usize)
                .map(|pos| (pos.0 as usize, pos.1 as usize))
                //If this is not the king, we are not allowed to move on the special squares
                .take_while(|pos| {
                    self[from] == WhiteKing || !Self::special_squares().contains(pos)
                })
        }).flatten().collect()
    }

    /// Moves the piece in the `from` position to the `to` position
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), MakeMoveError> {
        //Check if move is legal
        if !self.legal_moves(from).contains(&to) { return Err(IllegalMove); }
        //Check if move is made by the right player
        if self[from].player().unwrap() != self.turn { return Err(WrongPlayer); }

        //Move piece
        self[to] = self[from];
        self[from] = Empty;

        //Apply rules
        Self::rules().iter().for_each(|rule| {
            rule.make_move(self, from, to);
        });

        //Switch turn
        self.turn = self.turn.other();

        return Ok(());
    }
}

impl Index<(usize, usize)> for BoardConfiguration {
    type Output = FieldState;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        if pos.0 < 11 && pos.1 < 11 {
            &self.fields[pos.0][pos.1]
        } else {
            panic!("Invalid position");
        }
    }
}

impl Index<(isize, isize)> for BoardConfiguration {
    type Output = FieldState;

    fn index(&self, pos: (isize, isize)) -> &Self::Output {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < 11 && pos.1 < 11 {
            &self.fields[pos.0 as usize][pos.1 as usize]
        } else {
            panic!("Invalid position");
        }
    }
}

impl IndexMut<(usize, usize)> for BoardConfiguration {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        if pos.0 < 11 && pos.1 < 11 {
            &mut self.fields[pos.0][pos.1]
        } else {
            panic!("Invalid position");
        }
    }
}

impl IndexMut<(isize, isize)> for BoardConfiguration {
    fn index_mut(&mut self, pos: (isize, isize)) -> &mut Self::Output {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < 11 && pos.1 < 11 {
            &mut self.fields[pos.0 as usize][pos.1 as usize]
        } else {
            panic!("Invalid position");
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MakeMoveError {
    IllegalMove,
    WrongPlayer,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Player {
    White,
    Black,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            White => Black,
            Black => White
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum FieldState {
    WhiteKing,
    WhitePiece,
    BlackPiece,
    Empty,
}

impl FieldState {
    pub fn player(&self) -> Option<Player> {
        match &self {
            FieldState::WhiteKing => Some(White),
            FieldState::WhitePiece => Some(White),
            FieldState::BlackPiece => Some(Black),
            FieldState::Empty => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let board = BoardConfiguration::new();
        assert_eq!(board.fields[5][5], WhiteKing);
        assert_eq!(board.fields[5][4], WhitePiece);
        assert_eq!(board.fields[0][0], Empty);
        assert_eq!(board.fields[5][0], BlackPiece);
    }

    #[test]
    fn test_legal_moves() {
        let board = BoardConfiguration::new();
        assert_eq!(vec![(0, 0); 0], board.legal_moves((5, 0)));
        assert_eq!(vec![(2, 0), (1, 0), (3, 1), (3, 2), (3, 3), (3, 4)], board.legal_moves((3, 0)));
        assert_eq!(vec![(6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (4, 1), (3, 1), (2, 1), (1, 1), (0, 1), (5, 2)], board.legal_moves((5, 1)));
        assert_eq!(vec![(0, 0); 0], board.legal_moves((5, 2)));
        assert_eq!(vec![(6, 3), (7, 3), (8, 3), (9, 3), (4, 3), (3, 3), (2, 3), (1, 3), (5, 2)], board.legal_moves((5, 3)));
    }

    #[test]
    fn test_make_move() {
        let mut board = BoardConfiguration::new();
        assert_eq!(Err(IllegalMove), board.make_move((0, 0), (1, 0)));
        assert_eq!(board.fields[3][0], BlackPiece);
        assert_eq!(board.fields[2][0], Empty);
        assert_eq!(Ok(()), board.make_move((3, 0), (2, 0)));
        assert_eq!(board.fields[3][0], Empty);
        assert_eq!(board.fields[2][0], BlackPiece);
    }
}