use model_info_struct::model::budget_stream_graph::BudgetStreamGraphModelSpec;

use actix_web::web;
use model_info_struct::{
    enums::ModelSpecEnum,
    model::{
        campaign_list::CampaignListModelSpec, empire_list::EmpireListModelSpec,
        resource_summary_table::ResourceSummaryModelSpec,
    },
    ResourceClass, ResourceCode, ALL_RESOURCES,
};

use actix_web::HttpResponse;

use actix_web::Responder;

use game_data_controller::controller::GameModelController;

use actix_web::web::Data;

use actix_web::get;
use serde_derive::Deserialize;

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
    pub(crate) campaign_name: String,
    pub(crate) empire_name: String,
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
                ResourceClass::Nanites,
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

#[derive(Deserialize)]
pub struct ResourceSummaryRequest {
    campaign_name: String,
    empire_name: String,
    resource_list: String,
}
#[get("/{campaign_name}/{empire_name}/resourcesummary/{resource_list}")]
pub async fn resource_summary_data(
    s: Data<GameModelController>,
    resource_summary_request: web::Path<ResourceSummaryRequest>,
) -> impl Responder {
    let mut resources = vec![];

    for resource in ALL_RESOURCES {
        if resource_summary_request
            .resource_list
            .contains(resource.code())
        {
            resources.push(resource);
        }
    }

    log::info!(
        "Connection Request: Resource SummaryData for {}/{}/{:?}",
        resource_summary_request.campaign_name,
        resource_summary_request.empire_name,
        resources
    );
    match s.get_client(ModelSpecEnum::ResourceSummary(ResourceSummaryModelSpec {
        resources,
        campaign_name: resource_summary_request.campaign_name.to_string(),
        empire: resource_summary_request.empire_name.to_string(),
    })) {
        Some(client) => HttpResponse::Ok()
            .append_header(("content-type", "text/event-stream"))
            .append_header(("connection", "keep-alive"))
            .append_header(("cache-control", "no-cache"))
            .streaming(client),
        None => HttpResponse::NotFound().body(""),
    }
}
