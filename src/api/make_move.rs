use actix::Addr;
use actix_web::{get, post, web};

use crate::api::board_broadcast_server::{BoardBroadcast, ReceiveBoard};
use crate::api::make_move::MakeMoveResponse::{ERROR, SUCCESS};
use crate::state::TafliState;

#[get("/api/make_move")]
pub async fn make_move(state: web::Data<TafliState>, input: web::Query<MakeMoveInput>, bc: web::Data<Addr<BoardBroadcast>>) -> web::Json<MakeMoveResponse> {
    let mut board = state.board.lock().unwrap();

    let from = input.from();
    if from.is_err() { return web::Json(ERROR); }
    let to = input.to();
    if to.is_err() { return web::Json(ERROR); }

    let move_res = board.make_move(from.unwrap(), to.unwrap());
    if move_res.is_err() { return web::Json(ERROR); }
    bc.do_send(ReceiveBoard { board: *board });
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
        let x = inp.chars().next().ok_or(())? as isize - 'A' as isize;
        if x < 0 || x >= 11 { return Err(()); }
        let y = 11 - (inp.chars().nth(1).ok_or(())?.to_digit(10).ok_or(())? as isize);
        if y < 0 || y >= 11 { return Err(()); }
        Ok((x as usize, y as usize))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MakeMoveResponse {
    SUCCESS,
    ERROR,
}