use actix_session::Session;
use rand::{Rng, thread_rng};
use std::time::SystemTime;

use actix_web::{get, web};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::tafl::game::{Game, PlayerInfo};
use crate::api::in_game::game_broadcast_server::{board_broadcast, ReceiveGame};

pub trait UserIdSession {
    fn get_user_id(&self) -> usize;
}

impl UserIdSession for Session {
    fn get_user_id(&self) -> usize {
        if let Some(id) = self.get::<usize>("id").unwrap() {
            id
        } else {
            let mut rng = thread_rng();
            let id = rng.gen_range(0, 2usize.pow(50));
            self.set("id", id).unwrap();
            id
        }
    }
}

#[get("/api/identify")]
pub async fn identify(session: Session) -> web::Json<usize> {
    let userid = session.get_user_id();
    web::Json(userid)
}