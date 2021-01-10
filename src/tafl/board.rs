use crate::tafl::board::Player::{White, Black};
use crate::tafl::board::FieldState::{WhitePiece, WhiteKing, BlackPiece, Empty};
use std::convert::TryInto;
use crate::tafl::board::MakeMoveError::{IllegalMove, WrongPlayer};

struct BoardConfiguration {
    fields: [[FieldState; 11]; 11],
    turn: Player
}

impl BoardConfiguration {
    /// Creates a new board in the starting configuration
    fn new() -> Self {
        BoardConfiguration {
            fields: include_str!("../assets/tafl_board.txt").lines().map(|line| {
                line.chars().map(|char| {
                    match char {
                        'W' => WhitePiece,
                        'K' => WhiteKing,
                        'B' => BlackPiece,
                        'x' => Empty,
                        _ => unreachable!()
                    }
                }).collect::<Vec<_>>().as_slice().try_into().unwrap()
            }).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            turn: Player::Black
        }
    }

    /// Returns a Vec of all legal moves for the piece in the given position
    fn legal_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        //Empty tiles cannot move
        if self.fields[from.0][from.1] == Empty { return Vec::new() }

        //For each wind direction, iterate
        [(1,0), (-1, 0), (0, 1), (0, -1)].iter().map(|dir| {
            //How many times to move in this direction from the start
            (1..)
                //Map the count to the position in this direction
                .map(move |count| (from.0 as isize + count*dir.0, from.1 as isize + count*dir.1))

                //Take while the squares are in the board
                .take_while(|pos| {pos.0 >= 0 && pos.1 >= 0 && pos.0 < 11 && pos.1 < 11 })
                //Take while the squares are empty
                .take_while(|pos| { self.fields[pos.0 as usize][pos.1 as usize] == Empty })
                //If this is not the king, we are not allowed to move on the special squares
                .take_while(|pos| {
                    self.fields[from.0][from.1] == WhiteKing || ![(0, 0), (0, 10), (10, 0), (10, 10), (5,5)].contains(pos)
                })

                //Map to (usize, usize)
                .map(|pos| (pos.0 as usize, pos.1 as usize))
        }).flatten().collect()
    }

    /// Moves the piece in the `from` position to the `to` position
    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), MakeMoveError> {
        //Check if move is legal
        if !self.legal_moves(from).contains(&to) { return Err(IllegalMove); }
        //Check if move is made by the right player
        if self.fields[from.0][from.1].player().unwrap() != self.turn { return Err(WrongPlayer); }

        //Move piece
        self.fields[to.0][to.1] = self.fields[from.0][from.1];
        self.fields[from.0][from.1] = Empty;

        //Switch turn
        self.turn = self.turn.other();

        return Ok(());
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MakeMoveError {
    IllegalMove,
    WrongPlayer
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Player {
    White, Black
}

impl Player {
    fn other(&self) -> Player {
        match self {
            White => Black,
            Black => White
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FieldState {
    WhiteKing,
    WhitePiece,
    BlackPiece,
    Empty
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
        assert_eq!(board.fields[3][0], BlackPiece); assert_eq!(board.fields[2][0], Empty);
        assert_eq!(Ok(()), board.make_move((3, 0), (2, 0)));
        assert_eq!(board.fields[3][0], Empty); assert_eq!(board.fields[2][0], BlackPiece);
    }
}