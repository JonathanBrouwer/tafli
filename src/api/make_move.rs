use actix_web::{get, web};
use crate::state::TafliState;
use crate::api::make_move::MakeMoveResponse::{ERROR, SUCCESS};

#[get("/api/make_move")]
pub async fn make_move(state: web::Data<TafliState>, input: web::Query<MakeMoveInput>) -> web::Json<MakeMoveResponse> {
    let mut board = state.board.lock().unwrap();

    let from = input.from();
    if from.is_err() { return web::Json(ERROR) }
    let to = input.to();
    if to.is_err() { return web::Json(ERROR) }

    let move_res = board.make_move(from.unwrap(), to.unwrap());
    if move_res.is_err() { return web::Json(ERROR) }
    web::Json(SUCCESS)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MakeMoveInput {
    from: String,
    to: String
}

impl MakeMoveInput {
    fn from(&self) -> Result<(usize, usize), ()> {
        Self::convert_to_coord(&self.from)
    }

    fn to(&self) -> Result<(usize, usize), ()> {
        Self::convert_to_coord(&self.to)
    }

    fn convert_to_coord(inp: &String) -> Result<(usize, usize), ()> {
        let x = inp.chars().next().ok_or(())? as usize - 'A' as usize;
        let y = 11 - (inp.chars().nth(1).ok_or(())?.to_digit(10).ok_or(())? as usize);
        Ok((x, y))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MakeMoveResponse {
    SUCCESS,
    ERROR
}