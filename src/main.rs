use actix_files::Files;
use actix_web::{App, HttpServer};

use std::env;
use treasure_hunt::routes::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    env::set_var("RUST_BACKTRACE", "1");

    println!("Starting server...");

    // let bind_addr = ("0.0.0.0", 80);
    let bind_addr = ("127.0.0.1", 8080);

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(Files::new("*.css", "src/routes/home/"))
    })
    .bind(bind_addr)?
    .run()
    .await
}
