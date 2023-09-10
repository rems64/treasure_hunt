use actix_web::{App, HttpServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use std::fs::File;
use std::io::BufReader;
use treasure_hunt::routes::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    // env::set_var("RUST_BACKTRACE", "1");

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file(
    //         "/etc/letsencrypt/live/rfayet.me/privkey.pem",
    //         SslFiletype::PEM,
    //     )
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("/etc/letsencrypt/live/rfayet.me/fullchain.pem")
    //     .unwrap();
    // Create configuration
    let config = load_rustls_config();

    println!("Starting server...");

    let bind_addr = ("0.0.0.0", 80);

    HttpServer::new(|| App::new().service(home))
        // .bind(("0.0.0.0", 80))?
        // .bind(("127.0.0.1", 8080))?
        // .bind_openssl(("0.0.0.0", 80), builder)?
        .bind_rustls_021(bind_addr, config)?
        .run()
        .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
