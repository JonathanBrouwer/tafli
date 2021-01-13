use actix_web::{post, web};

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::api::in_game::game_broadcast_server;
use crate::api::in_game::game_broadcast_server::ReceiveGame;
use crate::api::in_game::make_move::MakeMoveResponse::{ERROR, SUCCESS};

#[post("/api/make_move")]
pub async fn make_move(input: web::Query<MakeMoveInput>) -> web::Json<MakeMoveResponse> {
    let mut games = GAMESTATE.full_games.lock().unwrap();
    let game = games.get_mut(&input.gameid).unwrap();

    let from = input.from();
    if from.is_err() { return web::Json(ERROR); }
    let to = input.to();
    if to.is_err() { return web::Json(ERROR); }

    let move_res = game.make_move(from.unwrap(), to.unwrap());
    if move_res.is_err() { return web::Json(ERROR); }
    game_broadcast_server::board_broadcast.do_send(ReceiveGame { game: game.clone() });
    web::Json(SUCCESS)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MakeMoveInput {
    gameid: usize,
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

    pub(crate) fn convert_to_coord(inp: &String) -> Result<(usize, usize), ()> {
        if !inp.contains(",") { return Err(()); }
        let x: usize = inp.split(",").next().ok_or(())?.parse().ok().ok_or(())?;
        let y: usize = inp.split(",").nth(1).ok_or(())?.parse().ok().ok_or(())?;
        return Ok((x, y));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MakeMoveResponse {
    SUCCESS,
    ERROR,
}