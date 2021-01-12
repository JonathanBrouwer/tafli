use std::collections::HashMap;

use actix::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::state;
use crate::tafl::game::Game;

lazy_static! {
    pub static ref board_broadcast: Addr<BoardBroadcast> = BoardBroadcast::new().start();
}

pub struct BoardBroadcast {
    sessions: HashMap<usize, Recipient<ReceiveGame>>,
    rng: ThreadRng,
}

impl BoardBroadcast {
    pub fn new() -> Self {
        BoardBroadcast {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl Actor for BoardBroadcast {
    type Context = Context<Self>;
}

impl Handler<Connect> for BoardBroadcast {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // send initial state
        let cur_game = state::state.game.lock().unwrap();
        let _ = msg.addr.do_send(ReceiveGame { game: cur_game.clone() });

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // send id back
        id
    }
}

impl Handler<Disconnect> for BoardBroadcast {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // remove address
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ReceiveGame> for BoardBroadcast {
    type Result = ();

    fn handle(&mut self, msg: ReceiveGame, _ctx: &mut Context<Self>) {
        self.sessions.values().for_each(|ses| {
            let _ = ses.do_send(ReceiveGame { game: msg.game.clone() });
        })
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ReceiveGame>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveGame {
    pub game: Game
}
