use crate::tafl::board::BoardConfiguration;
use crate::tafl::rules::rule::Rule;
use crate::tafl::board::FieldState::Empty;

pub struct CaptureRule;
impl Rule for CaptureRule {
    fn make_move(&self, board: &mut BoardConfiguration, from: (usize, usize), to: (usize, usize)) {
        //Check for direct captures
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|dir| {
            let p1 = ((to.0 as isize + dir.0), (to.1 as isize + dir.1));
            let p2 = ((to.0 as isize + dir.0 * 2), (to.1 as isize + dir.1 * 2));

            //If p2 is inside the map
            if p2.0 >= 0 && p2.0 < 11 && p2.1 >= 0 && p2.1 < 11 {
                let p1 = (p1.0 as usize, p1.1 as usize);
                let p2 = (p2.0 as usize, p2.1 as usize);
                //If both squares are not empty
                if board.fields[p1.0][p1.1] != Empty && (BoardConfiguration::special_squares().contains(&p2) || board.fields[p2.0][p2.1] != Empty) {
                    //If the piece at p1 is enemy, and the piece at p2 is friendly
                    if board.fields[p1.0][p1.1].player().unwrap() != board.turn && (BoardConfiguration::special_squares().contains(&p2) || board.fields[p2.0][p2.1].player().unwrap() == board.turn) {
                        //Capture p1
                        board.fields[p1.0][p1.1] = Empty;
                    }
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