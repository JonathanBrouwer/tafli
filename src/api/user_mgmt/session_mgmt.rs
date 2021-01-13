use actix_session::Session;
use rand::{thread_rng, Rng};

pub trait UserIdSession {
    fn get_user_id(&self) -> usize;
}

impl UserIdSession for Session {
    fn get_user_id(&self) -> usize {
        if let Some(id) = self.get::<usize>("id").unwrap() {
            id
        } else {
            let mut rng = thread_rng();
            let id = rng.gen();
            self.set("id", id).unwrap();
            id
        }
    }
}