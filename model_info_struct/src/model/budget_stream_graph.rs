use std::collections::HashMap;

use crate::{Model, ModelSpec};
use chrono::NaiveDate;
use game_data_info_struct::{EmpireData, ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

use super::date::Date;

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct BudgetStreamGraphModelSpec {
    pub resources: Vec<ResourceClass>,
    pub empire: String,
    pub campaign_name: String,
}

#[derive(Serialize, Debug)]
pub struct BudgetStreamGraphModel {
    list: Vec<(Date, Vec<f64>)>,
    spec: BudgetStreamGraphModelSpec,
}
impl Model for BudgetStreamGraphModel {
    type ModelSpec = BudgetStreamGraphModelSpec;
    type Representation = Vec<(Date, Vec<f64>)>;

    fn create(spec: Self::ModelSpec) -> Self {
        Self { list: vec![], spec }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        if game_data.campaign_name == self.spec.campaign_name {
            if let Some(empire) = game_data
                .empires
                .iter()
                .find(|e| e.name == self.spec.empire)
            {
                let form_model_point = self.form_model_point(empire);
                let element = (game_data.date.into(), form_model_point);
                match self
                    .list
                    .binary_search_by_key(&game_data.date, |(d, _v)| d.into())
                {
                    Ok(index) => {
                        self.list.remove(index);
                        self.list.insert(index, element.clone());
                    }
                    Err(index) => {
                        self.list.insert(index, element.clone());
                    }
                }
                Some(vec![element])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation> {
        match game_data_history.get(&self.spec.campaign_name) {
            Some(history) => {
                let mut list = vec![];
                for record in history {
                    if let Some(empire) = record.empires.iter().find(|e| e.name == self.spec.empire)
                    {
                        list.push((record.date.into(), self.form_model_point(empire)))
                    }
                }
                self.list = list;
                self.list.sort_by(|(a, _), (b, _)| {
                    let a: NaiveDate = a.into();
                    let b: NaiveDate = b.into();
                    a.cmp(&b)
                });
                Some(self.list.clone())
            }
            None => None,
        }
    }
}

impl BudgetStreamGraphModel {
    fn form_model_point(&self, game_data: &EmpireData) -> Vec<f64> {
        let mut ret = vec![];
        for resource in self.spec.resources.iter() {
            ret.push(
                game_data
                    .budget
                    .balance
                    .get(resource)
                    .unwrap()
                    .iter()
                    .fold(0f64, |acc, (_, x)| acc + x),
            )
        }
        ret
    }
}

impl ModelSpec for BudgetStreamGraphModelSpec {
    type Model = BudgetStreamGraphModel;
}
