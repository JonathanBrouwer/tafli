mod tafl;

use actix_web::{middleware::Logger, App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/", "./tafli/dist").show_files_listing().index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
