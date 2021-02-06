use std::time::SystemTime;

use actix_session::Session;
use actix_web::{post, web};
use rand::{Rng, thread_rng};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::api::game_mgmt::partial_game::PartialGame;
use crate::api::user_mgmt::session_mgmt::UserIdSession;
use crate::tafl::game::{Game, PlayerInfo};
use crate::api::in_game::game_broadcast_server::{board_broadcast, ReceiveGame};

#[post("/api/join_game")]
pub async fn join_game(mut input: web::Query<JoinGameInput>, session: Session) -> web::Json<bool> {
    let userid = session.get_user_id();
    if input.player_name == "" { input.player_name = "Anonymous".parse().unwrap() }

    //Find game
    let mut games = GAMESTATE.part_games.lock().unwrap();
    if !games.contains_key(&input.gameid) {
        return web::Json(false);
    }

    //Validate
    let pgame = games.get(&input.gameid).unwrap();
    if pgame.player.userid == userid {
        return web::Json(false);
    }

    //Generate colors
    let mut rng = thread_rng();
    let (mut wp, mut bp) = if rng.gen_bool(0.5) {
        (PlayerInfo {userid, name: input.player_name.clone()}, pgame.player.clone())
    } else {
        (pgame.player.clone(), PlayerInfo {userid, name: input.player_name.clone()})
    };

    //Start game
    let pgame = games.remove(&input.gameid).unwrap();
    let game = Game::new(
        pgame.game_id,
        wp,
        bp,
        (pgame.time_start, pgame.time_incr)
    );
    let mut games = GAMESTATE.full_games.lock().unwrap();
    games.insert(game.gameid, game.clone());
    board_broadcast.do_send(ReceiveGame{game});

    web::Json(true)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct JoinGameInput {
    player_name: String,
    gameid: usize
}