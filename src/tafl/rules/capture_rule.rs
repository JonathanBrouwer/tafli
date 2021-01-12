use crate::tafl::board::BoardConfiguration;
use crate::tafl::board::FieldState::{Empty, WhiteKing};
use crate::tafl::rules::rule::Rule;

pub struct CaptureRule;

impl Rule for CaptureRule {
    fn make_move(&self, board: &mut BoardConfiguration, _from: (usize, usize), to: (usize, usize)) {
        //Check for direct captures
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|dir| {
            let p1 = ((to.0 as isize + dir.0), (to.1 as isize + dir.1));
            let p2 = ((to.0 as isize + dir.0 * 2), (to.1 as isize + dir.1 * 2));

            //If p2 is inside the map
            if p2.0 >= 0 && p2.0 < 11 && p2.1 >= 0 && p2.1 < 11 {
                let p1 = (p1.0 as usize, p1.1 as usize);
                let p2 = (p2.0 as usize, p2.1 as usize);
                //If we can capture p1 with p2, and p1 is not the king
                if board.fields[p1.0][p1.1] != WhiteKing && board.can_capture_with(p1, p2) {
                    //Capture p1
                    board.fields[p1.0][p1.1] = Empty;
                }
            }
        });
    }
}

#[test]
fn test_capture() {
    let mut board = BoardConfiguration::new();
    assert_eq!(Ok(()), board.make_move((3, 0), (3, 4)));
    assert_eq!(Ok(()), board.make_move((7, 5), (7, 6)));
    assert_eq!(Ok(()), board.make_move((3, 10), (3, 6)));
    assert_eq!(Empty, board.fields[3][5]);
}

#[test]
fn test_not_capture_king() {
    let mut board = BoardConfiguration::new();
    let moves = [
        ((0, 3), (0, 1)),
        ((4, 4), (1, 4)),
        ((5, 1), (1, 1)),
        ((5, 3), (5, 1)),
        ((0, 1), (0, 3)),
        ((6, 4), (6, 1)),
        ((7, 0), (7, 2)),
        ((5, 4), (9, 4)),
        ((7, 2), (6, 2)),
        ((5, 5), (5, 4)),
        ((6, 2), (6, 4)),
        ((7, 5), (7, 4)),
        ((4, 0), (4, 4)),
        ((5, 1), (4, 1)),
        ((6, 0), (6, 4))
    ];
    for mv in moves.iter() {
        assert_eq!(Ok(()), board.make_move(mv.0, mv.1));
    }
    assert_eq!(WhiteKing, board.fields[5][4]);
}