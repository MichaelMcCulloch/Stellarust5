use std::path::PathBuf;

use actix_cors::Cors;

use actix_web::{
    dev::ServerHandle,
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use crossbeam::{channel::Sender, thread::Scope};
use game_data_controller::GameModelController;
use listenfd::ListenFd;
use model_info_struct::{
    enums::ModelSpecEnum,
    model::{
        budget_stream_graph::BudgetStreamGraphModelSpec, campaign_list::CampaignListModelSpec,
        empire_list::EmpireListModelSpec,
    },
    ResourceClass,
};
use serde_derive::Deserialize;
use stellarust::PROD_TEST_DATA_ROOT;

#[get("/")]
pub async fn index(s: Data<&str>) -> impl Responder {
    HttpResponse::Ok().body(String::from(*s.get_ref()))
}

#[get("/campaigns")]
pub async fn campaigns(s: Data<GameModelController>) -> impl Responder {
    log::info!("Connection Request: CampaignList");

    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("connection", "keep-alive"))
        .append_header(("cache-control", "no-cache"))
        .streaming(s.get_client(ModelSpecEnum::CampaignList(CampaignListModelSpec)))
}

#[get("/{campaign_name}/empires")]
pub async fn empires(
    s: Data<GameModelController>,
    campaign_name: web::Path<String>,
) -> impl Responder {
    log::info!("Connection Request: EmpireList for {}", campaign_name);
    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("connection", "keep-alive"))
        .append_header(("cache-control", "no-cache"))
        .streaming(s.get_client(ModelSpecEnum::EmpireList(EmpireListModelSpec {
            campaign_name: campaign_name.to_string(),
        })))
}
#[derive(Deserialize)]
pub struct BudgetRequest {
    campaign_name: String,
    empire_name: String,
}
#[get("/{campaign_name}/{empire_name}/budget")]
pub async fn budget_data(
    s: Data<GameModelController>,
    budget_request: web::Path<BudgetRequest>,
) -> impl Responder {
    log::info!(
        "Connection Request: BudgetData for {}/{}",
        budget_request.campaign_name,
        budget_request.empire_name
    );
    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("connection", "keep-alive"))
        .append_header(("cache-control", "no-cache"))
        .streaming(s.get_client(ModelSpecEnum::BudgetStreamGraph(
            BudgetStreamGraphModelSpec {
                resources: vec![
                    ResourceClass::Energy,
                    ResourceClass::Minerals,
                    ResourceClass::Alloys,
                ],
                campaign_name: budget_request.campaign_name.to_string(),
                empire: budget_request.empire_name.to_string(),
            },
        )))
}

pub async fn run_app(t: Sender<ServerHandle>, scope: &Scope<'_>) -> std::io::Result<()> {
    let game_data_controller = Data::new(GameModelController::create(
        &PathBuf::from(PROD_TEST_DATA_ROOT),
        scope,
    ));
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_header().allow_any_origin())
            .app_data(game_data_controller.clone())
            .service(index)
            .service(campaigns)
            .service(empires)
            .service(budget_data)
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
