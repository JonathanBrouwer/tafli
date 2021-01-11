use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::api::board_broadcast_server::{BoardBroadcast, Connect, Disconnect, ReceiveBoard};

pub struct WsGetBoard {
    id: usize,
    server: Addr<BoardBroadcast>,
}

impl Actor for WsGetBoard {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server
            .send(Connect { addr: ctx.address().recipient() })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop()
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.server
            .do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<ReceiveBoard> for WsGetBoard {
    type Result = ();

    fn handle(&mut self, msg: ReceiveBoard, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg.board).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGetBoard {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        _ctx: &mut Self::Context,
    ) {
        if msg.is_ok() {
            println!("Received {:?}", msg.unwrap())
        }
    }
}

pub async fn get_board(req: HttpRequest, stream: web::Payload, server: web::Data<Addr<BoardBroadcast>>) -> Result<HttpResponse, Error> {
    ws::start(WsGetBoard {
        id: 0,
        server: server.get_ref().clone(),
    }, &req, stream)
}