use std::collections::VecDeque;

use crate::tafl::board::BoardConfiguration;
use crate::tafl::board::FieldState::{Empty, BlackPiece, WhitePiece};
use crate::tafl::rules::rule::Rule;

pub struct ShieldWallRule;

impl ShieldWallRule {
    fn check(&self, board: &mut BoardConfiguration, to: (usize, usize), start: (usize, usize)) {
        if board.fields[start.0][start.1] == Empty { return; }
        if board.fields[start.0][start.1].player().unwrap() == board.fields[to.0][to.1].player().unwrap() { return; }

        //Find all pieces connected to the start, using a breath first search
        //Captured group will contain all of the walled off pieces
        let mut captured_group = Vec::new();
        //Queue will contain all the pieces yet to be processed
        let mut queue = VecDeque::new();
        queue.push_back(start);
        //Keep track if we failed
        let mut fail = false;

        while !queue.is_empty() {
            //Add this to the captured group
            let next = queue.pop_front().unwrap();
            captured_group.push(next);

            //For all its neighbours
            [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|dir| {
                //Check if nb is inside the board
                let nb = ((next.0 as isize + dir.0), (next.1 as isize + dir.1));
                if nb.0 >= 0 && nb.0 < 11 && nb.1 >= 0 && nb.1 < 11 {
                    let nb = (nb.0 as usize, nb.1 as usize);

                    //Check type of piece
                    match board.fields[nb.0][nb.1].player() {
                        //Can we use this neighbour as part of the wall?
                        _ if board.can_capture_with(start, nb) => {}
                        //If neighbour is air, group is not walled
                        None => fail = true,
                        //If neighbour is of the same team, and not seen earlier, add it to the queue
                        Some(player) if player == board.fields[start.0][start.1].player().unwrap() => {
                            if !queue.contains(&nb) && !captured_group.contains(&nb) {
                                queue.push_back(nb);
                            }
                        }
                        _ => unreachable!()
                    };
                }
            });
        }

        //If we didnt fail, this is a valid shield capture. Capture all the pieces
        if !fail {
            for piece in captured_group {
                board.fields[piece.0][piece.1] = Empty;
            }
        }
    }
}

impl Rule for ShieldWallRule {
    fn make_move(&self, board: &mut BoardConfiguration, _from: (usize, usize), to: (usize, usize)) {
        //Check top or bottom wall
        if to.1 == 0 || to.1 == 10 {
            //Check to the left
            if to.0 != 0 { self.check(board, to, (to.0-1, to.1))}
            //Check to the Right
            if to.0 != 10 { self.check(board, to, (to.0+1, to.1))}
        }

        //Check left or right wall
        if to.0 == 0 || to.0 == 10 {
            //Check to the top
            if to.1 != 0 { self.check(board, to, (to.0, to.1-1))}
            //Check to the bottom
            if to.1 != 10 { self.check(board, to, (to.0, to.1+1))}
        }
    }
}

#[test]
fn test_shield_wall() {
    let mut board = BoardConfiguration::from_file(include_str!("../../assets/shield_wall_test.txt"));
    assert_eq!(Ok(()), board.make_move((3, 0), (2, 0)));
    assert_eq!(Ok(()), board.make_move((5, 2), (10, 2)));
    assert_eq!(BlackPiece, board.fields[10][3]);
    assert_eq!(BlackPiece, board.fields[10][4]);
    assert_eq!(BlackPiece, board.fields[10][5]);
    assert_eq!(BlackPiece, board.fields[10][6]);
    assert_eq!(BlackPiece, board.fields[10][7]);
    assert_eq!(Ok(()), board.make_move((2, 0), (3, 0)));
    assert_eq!(Ok(()), board.make_move((5, 8), (10, 8)));
    assert_eq!(WhitePiece, board.fields[10][2]);
    assert_eq!(Empty, board.fields[10][3]);
    assert_eq!(Empty, board.fields[10][4]);
    assert_eq!(Empty, board.fields[10][5]);
    assert_eq!(Empty, board.fields[10][6]);
    assert_eq!(Empty, board.fields[10][7]);
    assert_eq!(WhitePiece, board.fields[10][8]);
}