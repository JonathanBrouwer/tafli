use actix_web::{get, web};

use crate::api::board_broadcast_server::{ReceiveBoard};
use crate::api::make_move::MakeMoveResponse::{ERROR, SUCCESS};
use crate::state;
use crate::api::board_broadcast_server;

#[get("/api/make_move")]
pub async fn make_move(input: web::Query<MakeMoveInput>) -> web::Json<MakeMoveResponse> {
    let mut board = state::state.board.lock().unwrap();

    let from = input.from();
    if from.is_err() { return web::Json(ERROR); }
    let to = input.to();
    if to.is_err() { return web::Json(ERROR); }

    let move_res = board.make_move(from.unwrap(), to.unwrap());
    if move_res.is_err() { return web::Json(ERROR); }
    board_broadcast_server::board_broadcast.do_send(ReceiveBoard { board: *board });
    web::Json(SUCCESS)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MakeMoveInput {
    from: String,
    to: String,
}

impl MakeMoveInput {
    fn from(&self) -> Result<(usize, usize), ()> {
        Self::convert_to_coord(&self.from)
    }

    fn to(&self) -> Result<(usize, usize), ()> {
        Self::convert_to_coord(&self.to)
    }

    fn convert_to_coord(inp: &String) -> Result<(usize, usize), ()> {
        if !inp.contains(",") { return Err(()); }
        let x: usize = inp.split(",").next().ok_or(())?.parse().ok().ok_or(())?;
        let y: usize = inp.split(",").nth(1).ok_or(())?.parse().ok().ok_or(())?;
        return Ok((x,y));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MakeMoveResponse {
    SUCCESS,
    ERROR,
}