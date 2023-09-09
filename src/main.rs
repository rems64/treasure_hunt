use actix_web::{App, HttpServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use treasure_hunt::routes::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("certs/nopass.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("certs/cert.pem")
    //     .unwrap();

    HttpServer::new(|| App::new().service(home))
        .bind(("0.0.0.0", 80))?
        // .bind(("127.0.0.1", 8080))?
        // .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
