use std::time::SystemTime;

use actix_session::Session;
use actix_web::{post, web};
use rand::{Rng, thread_rng};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::api::game_mgmt::partial_game::PartialGame;
use crate::api::user_mgmt::session_mgmt::UserIdSession;
use crate::tafl::game::PlayerInfo;

#[post("/api/create_game")]
pub async fn create_game(mut input: web::Query<CreateGameInput>, session: Session) -> web::Json<usize> {
    let user_id = session.get_user_id();
    if input.player_name == "" { input.player_name = "Anonymous".parse().unwrap() }

    let mut rng = thread_rng();
    let game_id = rng.gen_range(0, 2usize.pow(50));
    let game = PartialGame {
        game_id,
        player: PlayerInfo { userid: user_id, name: input.player_name.clone() },
        time_start: input.time_start,
        time_incr: input.time_incr,
        created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    let mut games = GAMESTATE.part_games.lock().unwrap();
    games.insert(game_id, game);

    web::Json(game_id)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CreateGameInput {
    player_name: String,
    time_start: usize,
    time_incr: usize,
}