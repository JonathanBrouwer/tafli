use std::collections::HashMap;

use actix::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::api::game_mgmt::game_mgmt::GAMESTATE;
use crate::tafl::game::Game;

lazy_static! {
    pub static ref board_broadcast: Addr<BoardBroadcast> = BoardBroadcast::new().start();
}

pub struct BoardBroadcast {
    sessions: HashMap<usize, Recipient<ReceiveGame>>,
    game_sessions: HashMap<usize, Vec<usize>>,
    rng: ThreadRng,
}

impl BoardBroadcast {
    pub fn new() -> Self {
        BoardBroadcast {
            sessions: HashMap::new(),
            game_sessions: HashMap::new(),
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
        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr.clone());

        // save the game this session is watching
        if !self.game_sessions.contains_key(&msg.gameid) {
            self.game_sessions.insert(msg.gameid, Vec::new());
        }
        self.game_sessions.get_mut(&msg.gameid).unwrap().push(id);

        // send initial state
        let games = GAMESTATE.full_games.lock().unwrap();
        if games.contains_key(&msg.gameid) {
            let _ = msg.addr.do_send(ReceiveGame { game: games.get(&msg.gameid).unwrap().clone() });
        }else {
            //TODO
        }

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
            let _ = ses.do_send(ReceiveGame {
                game: msg.game.clone()
            });
        })
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ReceiveGame>,
    pub gameid: usize
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
