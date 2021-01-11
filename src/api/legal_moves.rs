use actix_web::{get, web};

use crate::state;
use crate::api::make_move::MakeMoveInput;

#[get("/api/legal_moves")]
pub async fn legal_moves(input: web::Query<LegalMovesInput>) -> web::Json<LegalMovesResponse> {
    let board = state::state.board.lock().unwrap();

    let pos = input.pos();
    if pos.is_err() { return web::Json(LegalMovesResponse{ moves: Vec::new()}); }

    let moves = board.legal_moves(pos.unwrap());
    let moves = moves;
    web::Json(LegalMovesResponse{ moves })
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LegalMovesInput {
    pos: String
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