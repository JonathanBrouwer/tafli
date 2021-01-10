use actix_web::{get, web};
use super::super::tafl::board::BoardConfiguration;

#[get("/api/get_board")]
pub async fn get_board() -> web::Json<BoardConfiguration> {
    web::Json(BoardConfiguration::new())
}