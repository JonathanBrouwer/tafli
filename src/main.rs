extern crate actix_web;
extern crate actix_web_actors;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger, web};

mod tafl;
mod api;
pub mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(None);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/api/get_game", web::get().to(api::get_game::get_game))
            .service(api::make_move::make_move)
            .service(api::legal_moves::legal_moves)
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
