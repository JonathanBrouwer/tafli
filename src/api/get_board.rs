use actix_web::{get, web};
use super::super::tafl::board::BoardConfiguration;
use crate::state::TafliState;

#[get("/api/get_board")]
pub async fn get_board(state: web::Data<TafliState>) -> web::Json<BoardConfiguration> {
    web::Json(*state.board.lock().unwrap())
}