use std::collections::HashMap;

use crate::{Model, ModelSpec};
use game_data_info_struct::{ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

use super::date::Date;

#[derive(Eq, PartialEq, Hash, Serialize, Clone)]
pub struct BudgetStreamGraphModelSpec;

#[derive(Serialize)]
pub struct BudgetStreamGraphModel {
    list: Vec<(Date, Vec<f64>)>,
}
impl Model for BudgetStreamGraphModel {
    type ModelSpec = BudgetStreamGraphModelSpec;
    type Representation = Vec<(Date, Vec<f64>)>;

    fn create(_: Self::ModelSpec) -> Self {
        Self { list: vec![] }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        todo!()
    }

    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation> {
        todo!()
    }
}

impl ModelSpec for BudgetStreamGraphModelSpec {
    type Model = BudgetStreamGraphModel;
}
