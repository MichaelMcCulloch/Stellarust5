use std::{any::Any, collections::HashMap, hash::Hash};

use game_data_info_struct::{ModelDataPoint, ResourceClass};
use serde::Serialize;
use serde_derive::Serialize;

pub mod model;
use model::{
    budget_stream_graph::{BudgetStreamGraphModel, BudgetStreamGraphModelSpec},
    campaign_list::{CampaignInfoStruct, CampaignListModel, CampaignListModelSpec},
    date::Date,
};
pub mod enums;

pub trait Representation: Serialize {}
pub trait ModelSpec: Serialize + PartialEq + Eq + Hash {
    type Model: Model;
}

pub trait Model {
    type ModelSpec: ModelSpec;
    type Representation: Serialize;
    fn create(spec: Self::ModelSpec) -> Self;
    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation>;
    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation>;
}
