extern crate actix_web;
extern crate actix_web_actors;
#[macro_use]
extern crate serde;

use std::sync::Mutex;

use actix::Actor;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger, web};

use state::TafliState;

use crate::api::board_broadcast_server::BoardBroadcast;
use crate::tafl::board::BoardConfiguration;

mod tafl;
mod api;
pub mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let state = web::Data::new(TafliState {
        board: Mutex::new(BoardConfiguration::new())
    });

    let board_broadcast = web::Data::new(
        BoardBroadcast::new().start()
    );

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(None);

        App::new()
            .app_data(state.clone())
            .app_data(board_broadcast.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .route("/api/get_board", web::get().to(api::get_board::get_board))
            .service(api::make_move::make_move)
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
