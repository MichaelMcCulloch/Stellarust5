mod api;

use std::path::{Path, PathBuf};

use actix_cors::Cors;

use actix_web::{dev::Server, middleware, web::Data, App, HttpServer};

use actix_web_static_files::ResourceFiles;
use anyhow::Result;
use api::*;
use crossbeam::{channel::unbounded, thread::Scope};
use game_data_controller::controller::GameModelController;
use listenfd::ListenFd;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub async fn run_actix_server(scope: &Scope<'_>, game_data_root: &Path) -> Result<Server> {
    let game_data_controller = Data::new(GameModelController::create(
        &PathBuf::from(game_data_root),
        scope,
        unbounded(),
    ));
    let mut server = HttpServer::new(move || {
        let static_files = generate();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_header().allow_any_origin())
            .app_data(game_data_controller.clone())
            .service(campaigns)
            .service(empires)
            .service(budget_data)
            .service(resource_summary_data)
            .default_service(
                ResourceFiles::new("", static_files).resolve_not_found_to("index.html"),
            )
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        log::info!("{:?}", listener);
        server.listen(listener)?
    } else {
        log::info!("starting on 0.0.0.0:8000");
        server.bind("0.0.0.0:8000")?
    };

    let s = server.run();

    Ok(s)
}

/// You will need to generate the `key`.pem and the `cert`.pem by following https://actix.rs/docs/http2/
pub async fn run_actix_server_https(
    scope: &Scope<'_>,
    game_data_root: &Path,
    key: &Path,
    cert: &Path,
) -> Result<Server> {
    let game_data_controller = Data::new(GameModelController::create(
        &PathBuf::from(game_data_root),
        scope,
        unbounded(),
    ));

    let config = load_rustls_config(key, cert);
    let mut server = HttpServer::new(move || {
        let static_files = generate();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_header().allow_any_origin())
            .app_data(game_data_controller.clone())
            .service(campaigns)
            .service(empires)
            .service(budget_data)
            .service(resource_summary_data)
            .default_service(
                ResourceFiles::new("", static_files).resolve_not_found_to("index.html"),
            )
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        log::info!("{:?}", listener);
        server.listen_rustls(listener, config)?
    } else {
        log::info!("starting on 0.0.0.0:8000");
        server.bind_rustls("0.0.0.0:8000", config)?
    };

    let s = server.run();
    log::info!("Using HTTPS");

    Ok(s)
}

use std::{fs::File, io::BufReader};

fn load_rustls_config(key: &Path, cert: &Path) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

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
