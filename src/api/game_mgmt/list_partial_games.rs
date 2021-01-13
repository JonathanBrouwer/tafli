use actix_web::{get, web};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::api::game_mgmt::partial_game::PartialGame;

#[get("/api/list_partial_games")]
pub async fn list_partial_games() -> web::Json<Vec<PartialGame>> {
    let games = GAMESTATE.part_games.lock().unwrap();
    let games: Vec<PartialGame> = games.values().map(|x| x.clone()).collect();

    web::Json(games)
}