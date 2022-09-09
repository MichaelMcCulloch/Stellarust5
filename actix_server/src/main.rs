use std::{path::PathBuf, sync::Arc};

use actix_cors::Cors;
use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};

use campaign_controller::CampaignController;
use crossbeam::thread;
use listenfd::ListenFd;

#[get("/")]
pub async fn index(s: Data<&str>) -> impl Responder {
    HttpResponse::Ok().body(String::from(*s.get_ref()))
}

#[get("/campaigns")]
pub async fn campaign(s: Data<CampaignController>) -> impl Responder {
    log::info!("connection request");
    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(s.get_client())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let campaign_controller = Data::new(CampaignController::create(&PathBuf::from(
        "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/",
    )));
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_header().allow_any_origin())
            .app_data(campaign_controller.clone())
            .service(index)
            .service(campaign)
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        log::info!("{:?}", listener);
        server.listen(listener).unwrap()
    } else {
        log::info!("starting on 0.0.0.0:8000");
        server.bind("0.0.0.0:8000").unwrap()
    };

    server.run().await
}
