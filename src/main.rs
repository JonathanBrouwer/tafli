mod tafl;
mod api;

#[macro_use] extern crate serde;
#[macro_use] extern crate actix_web;

use actix_web::{middleware::Logger, App, HttpServer};
use actix_files as fs;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(None);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(api::get_board::get_board)
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
