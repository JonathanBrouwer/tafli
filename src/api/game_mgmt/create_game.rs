use actix_web::{post, web};

use crate::api::make_move::MakeMoveResponse::{ERROR, SUCCESS};
use actix_session::Session;
use rand::{thread_rng, Rng};
use crate::api::user_mgmt::session_mgmt::UserIdSession;


#[post("/api/create_game")]
pub async fn create_game(session: Session) -> web::Json<bool> {
    let user_id = session.get_user_id();



    web::Json(true)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CreateGameInput {
    player_name: String,
    time_start: usize,
    time_incr: usize
}