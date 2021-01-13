extern crate actix_web;
extern crate actix_web_actors;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use actix_cors::Cors;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{App, HttpServer, middleware::Logger, web, HttpRequest, Result};
use actix_files::NamedFile;
use std::path::PathBuf;

mod tafl;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(None);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            .route("/api/get_game", web::get().to(api::in_game::get_game::get_game))
            .service(api::game_mgmt::join_game::join_game)
            .service(api::game_mgmt::list_partial_games::list_partial_games)
            .service(api::game_mgmt::create_game::create_game)
            .service(api::in_game::make_move::make_move)
            .service(api::in_game::legal_moves::legal_moves)
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
            .default_service(web::get().to(index))
    })
        .bind(("localhost", 8000))?
        .run()
        .await
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./tafli/dist/index.html".parse().unwrap();
    println!("{:?}", path);
    Ok(NamedFile::open(path)?)
}
