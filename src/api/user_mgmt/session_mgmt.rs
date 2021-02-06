use actix_session::Session;
use actix_web::{get, web};
use rand::{Rng, thread_rng};

pub trait UserIdSession {
    fn get_user_id(&self) -> usize;
}

impl UserIdSession for Session {
    fn get_user_id(&self) -> usize {
        if let Some(id) = self.get::<usize>("id").unwrap() {
            id
        } else {
            let mut rng = thread_rng();
            let id = rng.gen_range(0, 2usize.pow(50));
            self.set("id", id).unwrap();
            id
        }
    }
}

#[get("/api/identify")]
pub async fn identify(session: Session) -> web::Json<usize> {
    let userid = session.get_user_id();
    web::Json(userid)
}