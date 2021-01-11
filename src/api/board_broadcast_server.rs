use std::collections::HashMap;

use actix::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::state;

use crate::tafl::board::BoardConfiguration;

lazy_static! {
    pub static ref board_broadcast: Addr<BoardBroadcast> = BoardBroadcast::new().start();
}

pub struct BoardBroadcast {
    sessions: HashMap<usize, Recipient<ReceiveBoard>>,
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
        println!("Someone joined");

        // send initial state
        let cur_board = *state::state.board.lock().unwrap();
        let _ = msg.addr.do_send(ReceiveBoard { board: cur_board });

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
        println!("Someone disconnected");

        // remove address
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ReceiveBoard> for BoardBroadcast {
    type Result = ();

    fn handle(&mut self, msg: ReceiveBoard, _ctx: &mut Context<Self>) {
        println!("Broadcast");

        self.sessions.values().for_each(|ses| {
            let _ = ses.do_send(ReceiveBoard { board: msg.board });
        })
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ReceiveBoard>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveBoard {
    pub board: BoardConfiguration
}