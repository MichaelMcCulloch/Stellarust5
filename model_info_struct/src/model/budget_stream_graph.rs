use crate::{Model, ModelSpec};
use game_data_info_struct::{ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

use super::date::Date;

#[derive(Eq, PartialEq, Hash, Serialize, Clone)]
pub struct BudgetStreamGraphModelSpec;

#[derive(Serialize)]
pub struct BudgetStreamGraphModel {
    resource_list: Vec<ResourceClass>,
    list: Vec<(Date, Vec<f64>)>,
}
impl Model for BudgetStreamGraphModel {
    type ModelSpec = BudgetStreamGraphModelSpec;
    type Representation = Vec<(Date, Vec<f64>)>;

    fn create(_: Self::ModelSpec) -> Self {
        Self {
            resource_list: vec![],
            list: vec![],
        }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Self::Representation {
        todo!()
    }

    fn update_all(&mut self, game_data_history: &Vec<ModelDataPoint>) -> Self::Representation {
        todo!()
    }
}

impl ModelSpec for BudgetStreamGraphModelSpec {
    type Model = BudgetStreamGraphModel;
}
