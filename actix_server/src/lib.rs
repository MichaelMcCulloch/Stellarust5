use std::path::PathBuf;

use actix_cors::Cors;

use actix_web::{
    dev::Server,
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use anyhow::Result;
use crossbeam::{channel::unbounded, thread::Scope};
use game_data_controller::controller::GameModelController;
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
pub async fn index(s: Data<GameModelController>) -> impl Responder { 
    log::info!("Connection Request: CampaignList");

    match s.get_client(ModelSpecEnum::CampaignList(CampaignListModelSpec)) {
        Some(client) => HttpResponse::Ok()
            .append_header(("content-type", "text/event-stream"))
            .append_header(("connection", "keep-alive"))
            .append_header(("cache-control", "no-cache"))
            .streaming(client),
        None => HttpResponse::NotFound().body(""),
    }
}

#[get("/campaigns")]
pub async fn campaigns(s: Data<GameModelController>) -> impl Responder {
    log::info!("Connection Request: CampaignList");

    match s.get_client(ModelSpecEnum::CampaignList(CampaignListModelSpec)) {
        Some(client) => HttpResponse::Ok()
            .append_header(("content-type", "text/event-stream"))
            .append_header(("connection", "keep-alive"))
            .append_header(("cache-control", "no-cache"))
            .streaming(client),
        None => HttpResponse::NotFound().body(""),
    }
}

#[get("/{campaign_name}/empires")]
pub async fn empires(
    s: Data<GameModelController>,
    campaign_name: web::Path<String>,
) -> impl Responder {
    log::info!("Connection Request: EmpireList for {}", campaign_name);
    match s.get_client(ModelSpecEnum::EmpireList(EmpireListModelSpec {
        campaign_name: campaign_name.to_string(),
    })) {
        Some(client) => HttpResponse::Ok()
            .append_header(("content-type", "text/event-stream"))
            .append_header(("connection", "keep-alive"))
            .append_header(("cache-control", "no-cache"))
            .streaming(client),
        None => HttpResponse::NotFound().body(""),
    }
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
    match s.get_client(ModelSpecEnum::BudgetStreamGraph(
        BudgetStreamGraphModelSpec {
            resources: vec![
                ResourceClass::Energy,
                ResourceClass::Minerals,
                ResourceClass::Alloys,
            ],
            campaign_name: budget_request.campaign_name.to_string(),
            empire: budget_request.empire_name.to_string(),
        },
    )) {
        Some(client) => HttpResponse::Ok()
            .append_header(("content-type", "text/event-stream"))
            .append_header(("connection", "keep-alive"))
            .append_header(("cache-control", "no-cache"))
            .streaming(client),
        None => HttpResponse::NotFound().body(""),
    }
}

pub async fn run_actix_server(scope: &Scope<'_>) -> Result<Server> {
    let game_data_controller = Data::new(GameModelController::create(
        &PathBuf::from(PROD_TEST_DATA_ROOT),
        scope,
        unbounded()
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

#[cfg(test)]
mod tests {
    use std::{time::Duration};

    use actix_rt::pin;
    use actix_web::{
        body::MessageBody,
        test::{self},
    };
    use chrono::NaiveDate;
    use crossbeam::{thread, channel::unbounded};
    use futures::{executor, future};
    use game_data_info_struct::ModelDataPoint;
    use stellarust::PROD_TEST_EMPTY_FOLDER;

    use super::*;
    #[actix_rt::test]
    async fn test_name() {
        thread::scope(|scope| {
            std::env::set_var("RUST_LOG", "warn");
            env_logger::init();
            let (sender, receiver) = unbounded();
            let game_data_controller = Data::new(GameModelController::create(
                &PathBuf::from(PROD_TEST_EMPTY_FOLDER),
                scope,
                (sender.clone(), receiver)
            ));
            let app = executor::block_on(test::init_service(
                App::new()
                    .app_data(game_data_controller.clone())
                    .service(campaigns),
            ));

            sender.send(ModelDataPoint { campaign_name: "TEST_CAMPAIGN".to_string(), date: NaiveDate::MAX.into() , empires: vec![] }).unwrap();
            std::thread::sleep(Duration::from_millis(50));

            let req = test::TestRequest::get().uri("/campaigns").to_request();

            let resp = executor::block_on(test::call_service(&app, req));
            assert!(resp.status().is_success());
            let body = resp.into_body();
            pin!(body);
            let actual = vec![
                executor::block_on(future::poll_fn(|cx| body.as_mut().poll_next(cx))).unwrap().unwrap(),
                executor::block_on(future::poll_fn(|cx| body.as_mut().poll_next(cx))).unwrap().unwrap()
            ];

            let expected = vec![
                web::Bytes::from_static(b"event: connected\ndata: connected\n\n"),     
                web::Bytes::from_static(b"event: message\ndata: {\"CampaignList\":[{\"campaign_name\":\"TEST_CAMPAIGN\",\"empire_list\":[]}]}\n\n")];

            assert_eq!(
                expected,
                actual
            );
          
        })
        .unwrap();
    }
}
