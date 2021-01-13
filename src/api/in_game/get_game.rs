use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::api::in_game::game_broadcast_server::{Connect, Disconnect, ReceiveGame};
use crate::api::in_game::game_broadcast_server::board_broadcast;

pub struct WsGetGame {
    address: usize,
    gameid: usize,
}

impl Actor for WsGetGame {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        board_broadcast
            .send(Connect { addr: ctx.address().recipient(), gameid: self.gameid })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.address = res,
                    _ => ctx.stop()
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        board_broadcast
            .do_send(Disconnect { id: self.address });
        Running::Stop
    }
}

impl Handler<ReceiveGame> for WsGetGame {
    type Result = ();

    fn handle(&mut self, msg: ReceiveGame, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg.game).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGetGame {
    fn handle(
        &mut self,
        _msg: Result<ws::Message, ws::ProtocolError>,
        _ctx: &mut Self::Context,
    ) {}
}

pub async fn get_game(req: HttpRequest, stream: web::Payload, input: web::Query<GetGameParams>) -> Result<HttpResponse, Error> {
    ws::start(WsGetGame {
        address: 0,
        gameid: input.id,
    }, &req, stream)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct GetGameParams {
    id: usize
}