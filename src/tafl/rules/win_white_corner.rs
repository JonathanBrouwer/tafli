use crate::tafl::board::FieldState::WhiteKing;
use crate::tafl::board::Player::White;
use crate::tafl::game::Game;
use crate::tafl::game::GameStatus::Won;
use crate::tafl::rules::rule::Rule;

pub struct WinWhiteCorner;

impl Rule for WinWhiteCorner {
    fn make_move(&self, game: &mut Game, _from: (usize, usize), to: (usize, usize)) {
        let winning_squares = [(0, 0), (0, 10), (10, 0), (10, 10)];
        if game.board[to] == WhiteKing && winning_squares.contains(&to) {
            //White wins!
            game.status = Won(White, String::from("King reached the corner"));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tafl::board::Player::White;
    use crate::tafl::game::Game;
    use crate::tafl::game::GameStatus::Won;

    #[test]
    fn test_king_escape() {
        let mut game = Game::from_file(include_str!("../../assets/test_win_white_corner.txt"));
        assert_eq!(Ok(()), game.make_move((3, 0), (2, 0)));
        assert_eq!(Ok(()), game.make_move((1, 0), (0, 0)));
        assert_eq!(game.status, Won(White, String::from("King reached the corner")))
    }
}