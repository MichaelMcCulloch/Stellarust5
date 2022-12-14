use crate::{
    model::{
        budget_stream_graph::{BudgetStreamGraphModel, BudgetStreamGraphModelSpec},
        campaign_list::{CampaignInfoStruct, CampaignListModel, CampaignListModelSpec, Empire},
        empire_list::{EmpireListModel, EmpireListModelSpec},
        resource_summary_table::{LineChartData, ResourceSummaryModel, ResourceSummaryModelSpec},
    },
    Model, ModelSpec, Representation,
};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{date::Date, model::ModelDataPoint, resource::ResourceClass};
use serde_derive::Serialize;
use std::hash::BuildHasherDefault;

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub enum ModelSpecEnum {
    CampaignList(CampaignListModelSpec),
    EmpireList(EmpireListModelSpec),
    BudgetStreamGraph(BudgetStreamGraphModelSpec),
    ResourceSummary(ResourceSummaryModelSpec),
    BudgetMonthlySankyDiagram(ResourceClass),
}
#[derive(Debug)]
pub enum ModelEnum {
    CampaignList(CampaignListModel),
    EmpireList(EmpireListModel),
    BudgetStreamGraph(BudgetStreamGraphModel),
    ResourceSummary(ResourceSummaryModel),
    BudgetMonthlySankyDiagram(),
}

#[derive(Serialize, Debug)]
pub enum RepresentationEnum {
    CampaignList(Vec<CampaignInfoStruct>),
    EmpireList(Vec<Empire>),
    BudgetStreamGraph(Vec<(Date, Vec<f64>)>),
    ResourceSummary(LineChartData),
}
impl Model for ModelEnum {
    type ModelSpec = ModelSpecEnum;
    type Representation = RepresentationEnum;

    fn create(spec: Self::ModelSpec) -> Self {
        match spec {
            ModelSpecEnum::CampaignList(spec) => {
                ModelEnum::CampaignList(CampaignListModel::create(spec))
            }
            ModelSpecEnum::BudgetStreamGraph(spec) => {
                ModelEnum::BudgetStreamGraph(BudgetStreamGraphModel::create(spec))
            }
            ModelSpecEnum::BudgetMonthlySankyDiagram(_) => todo!(),
            ModelSpecEnum::EmpireList(spec) => ModelEnum::EmpireList(EmpireListModel::create(spec)),
            ModelSpecEnum::ResourceSummary(spec) => {
                ModelEnum::ResourceSummary(ResourceSummaryModel::create(spec))
            }
        }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        match self {
            ModelEnum::CampaignList(model) => model
                .update(game_data)
                .map(|rep| RepresentationEnum::CampaignList(rep)),
            ModelEnum::BudgetStreamGraph(model) => model
                .update(game_data)
                .map(|rep| RepresentationEnum::BudgetStreamGraph(rep)),
            ModelEnum::BudgetMonthlySankyDiagram() => todo!(),
            ModelEnum::EmpireList(model) => model
                .update(game_data)
                .map(|rep| RepresentationEnum::EmpireList(rep)),
            ModelEnum::ResourceSummary(model) => model
                .update(game_data)
                .map(|rep| RepresentationEnum::ResourceSummary(rep)),
        }
    }

    fn update_all(
        &mut self,
        game_data_history: &DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>,
    ) -> Option<Self::Representation> {
        match self {
            ModelEnum::CampaignList(model) => model
                .update_all(game_data_history)
                .map(|rep| RepresentationEnum::CampaignList(rep)),
            ModelEnum::BudgetStreamGraph(model) => model
                .update_all(game_data_history)
                .map(|rep| RepresentationEnum::BudgetStreamGraph(rep)),
            ModelEnum::BudgetMonthlySankyDiagram() => todo!(),
            ModelEnum::EmpireList(model) => model
                .update_all(game_data_history)
                .map(|rep| RepresentationEnum::EmpireList(rep)),
            ModelEnum::ResourceSummary(model) => model
                .update_all(game_data_history)
                .map(|rep| RepresentationEnum::ResourceSummary(rep)),
        }
    }

    fn get(&self) -> Self::Representation {
        match self {
            ModelEnum::CampaignList(model) => RepresentationEnum::CampaignList(model.get()),
            ModelEnum::BudgetStreamGraph(model) => {
                RepresentationEnum::BudgetStreamGraph(model.get())
            }
            ModelEnum::BudgetMonthlySankyDiagram() => todo!(),
            ModelEnum::EmpireList(model) => RepresentationEnum::EmpireList(model.get()),
            ModelEnum::ResourceSummary(model) => RepresentationEnum::ResourceSummary(model.get()),
        }
    }
}
impl ModelSpec for ModelSpecEnum {
    type Model = ModelEnum;
}

impl Representation for RepresentationEnum {}
