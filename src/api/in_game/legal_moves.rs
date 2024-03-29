use actix_web::{get, web};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::api::in_game::make_move::MakeMoveInput;

#[get("/api/legal_moves")]
pub async fn legal_moves(input: web::Query<LegalMovesInput>) -> web::Json<LegalMovesResponse> {
    let mut games = GAMESTATE.full_games.lock().unwrap();
    let game = games.get_mut(&input.gameid);
    if game.is_none() { return web::Json(LegalMovesResponse { moves: Vec::new() }); }
    let game = game.unwrap();

    let pos = input.pos();
    if pos.is_err() { return web::Json(LegalMovesResponse { moves: Vec::new() }); }

    let moves = game.legal_moves(pos.unwrap());
    web::Json(LegalMovesResponse { moves })
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LegalMovesInput {
    gameid: usize,
    pos: String,
}

impl LegalMovesInput {
    pub fn pos(&self) -> Result<(usize, usize), ()> {
        MakeMoveInput::convert_to_coord(&self.pos)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LegalMovesResponse {
    moves: Vec<(usize, usize)>
}