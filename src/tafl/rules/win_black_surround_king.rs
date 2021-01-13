use crate::tafl::board::FieldState::{WhiteKing};
use crate::tafl::game::Game;
use crate::tafl::rules::rule::Rule;
use crate::tafl::game::GameStatus::Won;
use crate::tafl::board::Player::Black;

pub struct WinBlackSurroundKing;

impl Rule for WinBlackSurroundKing {
    fn make_move(&self, game: &mut Game, _from: (usize, usize), to: (usize, usize)) {
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|dir| {
            let king = ((to.0 as isize + dir.0), (to.1 as isize + dir.1));
            //Check if king is not on the edge
            if king.0 >= 1 && king.0 < 10 && king.1 >= 1 && king.1 < 10 && game.board[king] == WhiteKing {
                //Check if king is surrounded
                if [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().all(|dir| {
                    let nb = (king.0 + dir.0, king.1 + dir.1);
                    game.board.can_capture_with((king.0 as usize, king.1 as usize), (nb.0 as usize, nb.1 as usize))
                }) {
                    game.status = Won(Black, String::from("King was surrounded"));
                    game.prev_move_info.captures.push((king.0 as usize, king.1 as usize));
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use crate::tafl::game::Game;
    use crate::tafl::board::Player::{White, Black};
    use crate::tafl::game::GameStatus::{Won, Playing};

    #[test]
    fn test_king_surround() {
        let mut game = Game::from_file(include_str!("../../assets/test_black_king_surround.txt"));
        assert_eq!(Ok(()), game.make_move((0, 2), (0,1)));
        assert_eq!(game.status, Won(Black, String::from("King was surrounded")))
    }

    #[test]
    fn test_king_with_middle() {
        let mut game = Game::from_file(include_str!("../../assets/test_black_king_surround.txt"));
        assert_eq!(Ok(()), game.make_move((5,8), (5,7)));
        assert_eq!(game.status, Won(Black, String::from("King was surrounded")))
    }

    #[test]
    fn test_king_safe_edge() {
        let mut game = Game::from_file(include_str!("../../assets/test_black_king_surround.txt"));
        assert_eq!(Ok(()), game.make_move((1,0), (2,0)));
        assert_eq!(Ok(()), game.make_move((1,1), (0,1)));
        assert_eq!(Ok(()), game.make_move((2,1), (1,1)));
        assert_eq!(game.status, Playing)
    }
}