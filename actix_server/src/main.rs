use std::path::PathBuf;

use actix_cors::Cors;

use actix_web::{
    dev::ServerHandle, get, middleware, rt, web::Data, App, HttpResponse, HttpServer, Responder,
};

use crossbeam::{
    channel::{unbounded, Sender},
    thread::{self, Scope},
};
use game_data_controller::GameModelController;
use listenfd::ListenFd;
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    model::campaign_list::CampaignListModelSpec,
};

#[get("/")]
pub async fn index(s: Data<&str>) -> impl Responder {
    HttpResponse::Ok().body(String::from(*s.get_ref()))
}

#[get("/campaigns")]
pub async fn campaigns(s: Data<GameModelController>) -> impl Responder {
    log::info!("connection request");
    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("connection", "keep-alive"))
        .append_header(("cache-control", "no-cache"))
        .streaming(s.get_client(ModelSpecEnum::CampaignList(CampaignListModelSpec)))
}

fn main() -> Result<(), Box<(dyn std::any::Any + Send + 'static)>> {
    thread::scope(|scope| {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();

        let (t, r) = unbounded();

        scope.spawn(|scope| -> Result<_, std::io::Error> {
            let server_future = run_app(t, scope);
            rt::System::new().block_on(server_future)
        });

        let _server_handle = r.recv().unwrap();

        // rt::System::new().block_on(server_handle.stop(true))
    })
}

async fn run_app(t: Sender<ServerHandle>, scope: &Scope<'_>) -> std::io::Result<()> {
    let game_data_controller = Data::new(GameModelController::create(
        &PathBuf::from("/home/michael/Dev/Stellarust/stellarust5/production_data/3.4.5.95132"),
        scope,
    ));
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_header().allow_any_origin())
            // .app_data(campaign_controller.clone())
            .app_data(game_data_controller.clone())
            .service(index)
            .service(campaigns)
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        log::info!("{:?}", listener);
        server.listen(listener).unwrap()
    } else {
        log::info!("starting on 0.0.0.0:8000");
        server.bind("0.0.0.0:8000").unwrap()
    };

    let s = server.run();

    let _ = t.send(s.handle());

    s.await
}
