use crate::tafl::board::FieldState::{BlackPiece, Empty, WhiteKing, WhitePiece};
use crate::tafl::board::Player::{Black, White};
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub(crate) fn other(&self) -> Player {
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
}