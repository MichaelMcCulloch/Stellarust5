use game_data_info_struct::{ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

use crate::{
    model::{
        budget_stream_graph::{BudgetStreamGraphModel, BudgetStreamGraphModelSpec},
        campaign_list::{CampaignInfoStruct, CampaignListModel, CampaignListModelSpec},
        date::Date,
    },
    Model, ModelSpec, Representation,
};

#[derive(Eq, PartialEq, Hash, Serialize, Clone)]
pub enum ModelSpecEnum {
    CampaignList(CampaignListModelSpec),
    BudgetStreamGraph(BudgetStreamGraphModelSpec),
    BudgetMonthlySankyDiagram(ResourceClass),
}
pub enum ModelEnum {
    CampaignList(CampaignListModel),
    BudgetStreamGraph(BudgetStreamGraphModel),
    BudgetMonthlySankyDiagram(),
}

#[derive(Serialize)]
pub enum RepresentationEnum {
    CampaignList(Vec<CampaignInfoStruct>),
    BudgetStreamGraph(Vec<(Date, Vec<f64>)>),
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
        }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Self::Representation {
        match self {
            ModelEnum::CampaignList(model) => {
                RepresentationEnum::CampaignList(model.update(game_data))
            }
            ModelEnum::BudgetStreamGraph(model) => {
                RepresentationEnum::BudgetStreamGraph(model.update(game_data))
            }
            ModelEnum::BudgetMonthlySankyDiagram() => todo!(),
        }
    }

    fn update_all(&mut self, game_data_history: &Vec<ModelDataPoint>) -> Self::Representation {
        match self {
            ModelEnum::CampaignList(model) => {
                RepresentationEnum::CampaignList(model.update_all(game_data_history))
            }
            ModelEnum::BudgetStreamGraph(model) => {
                RepresentationEnum::BudgetStreamGraph(model.update_all(game_data_history))
            }
            ModelEnum::BudgetMonthlySankyDiagram() => todo!(),
        }
    }
}
impl ModelSpec for ModelSpecEnum {
    type Model = ModelEnum;
}

impl Representation for RepresentationEnum {}
