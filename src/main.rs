use std::sync::Mutex;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger, web};

use state::TafliState;
use crate::tafl::board::BoardConfiguration;

mod tafl;
mod api;
pub mod state;

#[macro_use] extern crate serde;
extern crate actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let state = web::Data::new(TafliState {
        board: Mutex::new(BoardConfiguration::new())
    });

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(None);
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .service(api::get_board::get_board)
            .service(api::make_move::make_move)
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
