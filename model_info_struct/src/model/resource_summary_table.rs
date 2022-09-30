use std::hash::BuildHasherDefault;

use crate::{Model, ModelSpec};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{date::Date, EmpireData, Index, ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct ResourceSummaryTableModelSpec {
    pub resources: Vec<ResourceClass>,
    pub empire: String,
    pub campaign_name: String,
}

#[derive(Serialize, Debug)]
pub struct ResourceSummaryTableModel {
    list: Vec<(Date, Vec<f64>)>,
    spec: ResourceSummaryTableModelSpec,
}
impl Model for ResourceSummaryTableModel {
    type ModelSpec = ResourceSummaryTableModelSpec;
    type Representation = Vec<(Date, Vec<f64>)>;

    fn create(spec: Self::ModelSpec) -> Self {
        Self { list: vec![], spec }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        match self.form_model_point(game_data) {
            Some(data) => {
                match self
                    .list
                    .binary_search_by_key(&Date::from(game_data.date), |(d, _v)| *d)
                {
                    Ok(index) => {
                        self.list.remove(index);
                        self.list.insert(index, data.clone());
                    }
                    Err(index) => {
                        self.list.insert(index, data.clone());
                    }
                }
                Some(vec![data])
            }
            None => None,
        }
    }

    fn update_all(
        &mut self,
        game_data_history: &DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>,
    ) -> Option<Self::Representation> {
        match game_data_history.get(&self.spec.campaign_name) {
            Some(history) => {
                for record in &*history {
                    self.update(&record);
                }

                Some(self.list.clone())
            }
            None => None,
        }
    }

    fn get(&self) -> Self::Representation {
        self.list.clone()
    }
}

impl ResourceSummaryTableModel {
    fn form_model_point(&self, game_data: &ModelDataPoint) -> Option<(Date, Vec<f64>)> {
        if game_data.campaign_name != self.spec.campaign_name {
            None
        } else if let Some(empire) = game_data
            .empires
            .iter()
            .find(|e| e.name == self.spec.empire)
        {
            Some((Date::from(game_data.date), self.get_resource_values(empire)))
        } else {
            None
        }
    }
    fn get_resource_values(&self, empire_data: &EmpireData) -> Vec<f64> {
        let mut ret = vec![];
        for resource in self.spec.resources.iter() {
            ret.push(*empire_data.resources.index(&resource));
        }
        ret
    }
}

impl ModelSpec for ResourceSummaryTableModelSpec {
    type Model = ResourceSummaryTableModel;
}

#[cfg(test)]
mod tests {}
