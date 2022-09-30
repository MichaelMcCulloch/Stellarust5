use std::hash::BuildHasherDefault;

use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{date::Date, ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

use crate::{
    model::{
        self,
        budget_stream_graph::{BudgetStreamGraphModel, BudgetStreamGraphModelSpec},
        campaign_list::{CampaignInfoStruct, CampaignListModel, CampaignListModelSpec, Empire},
        empire_list::{EmpireListModel, EmpireListModelSpec},
        resource_summary_table::{ResourceSummaryTableModel, ResourceSummaryTableModelSpec},
    },
    Model, ModelSpec, Representation,
};

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub enum ModelSpecEnum {
    CampaignList(CampaignListModelSpec),
    EmpireList(EmpireListModelSpec),
    BudgetStreamGraph(BudgetStreamGraphModelSpec),
    ResourceSummaryTable(ResourceSummaryTableModelSpec),
    BudgetMonthlySankyDiagram(ResourceClass),
}
#[derive(Debug)]
pub enum ModelEnum {
    CampaignList(CampaignListModel),
    EmpireList(EmpireListModel),
    BudgetStreamGraph(BudgetStreamGraphModel),
    ResourceSummaryTable(ResourceSummaryTableModel),
    BudgetMonthlySankyDiagram(),
}

#[derive(Serialize, Debug)]
pub enum RepresentationEnum {
    CampaignList(Vec<CampaignInfoStruct>),
    EmpireList(Vec<Empire>),
    BudgetStreamGraph(Vec<(Date, Vec<f64>)>),
    ResourceSummaryTable(Vec<(Date, Vec<f64>)>),
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
            ModelSpecEnum::ResourceSummaryTable(spec) => {
                ModelEnum::ResourceSummaryTable(ResourceSummaryTableModel::create(spec))
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
            ModelEnum::ResourceSummaryTable(model) => model
                .update(game_data)
                .map(|rep| RepresentationEnum::ResourceSummaryTable(rep)),
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
            ModelEnum::ResourceSummaryTable(model) => model
                .update_all(game_data_history)
                .map(|rep| RepresentationEnum::ResourceSummaryTable(rep)),
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
            ModelEnum::ResourceSummaryTable(model) => {
                RepresentationEnum::ResourceSummaryTable(model.get())
            }
        }
    }
}
impl ModelSpec for ModelSpecEnum {
    type Model = ModelEnum;
}

impl Representation for RepresentationEnum {}
