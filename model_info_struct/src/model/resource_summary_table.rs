use std::hash::{BuildHasherDefault, Hash};

use crate::{Model, ModelSpec};
use chrono::NaiveDate;
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{EmpireData, Index, ModelDataPoint, ResourceClass};
use serde_derive::Serialize;

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct ResourceSummaryModelSpec {
    pub resources: Vec<ResourceClass>,
    pub empire: String,
    pub campaign_name: String,
}

/// ResourceSummary
/// Intended to be displayable with a `LineChart`
/// Representation = `List<(Days since 2200.01.01, Resource Values)>`
#[derive(Serialize, Debug)]
pub struct ResourceSummaryModel {
    list: Vec<(u64, Vec<f64>)>,
    spec: ResourceSummaryModelSpec,
}
impl Model for ResourceSummaryModel {
    type ModelSpec = ResourceSummaryModelSpec;
    type Representation = Vec<(u64, Vec<f64>)>; // This is not very readable. TODO: Instead let's wrap it in a struct, then employ serde::serialize_with to turn a code acceptable copy into the representation google charts needs

    fn create(spec: Self::ModelSpec) -> Self {
        Self { list: vec![], spec }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        match self.form_model_point(game_data) {
            Some(data) => {
                let days_from_start = game_data
                    .date
                    .signed_duration_since(NaiveDate::from_ymd(2200, 01, 01))
                    .num_days() as u64;
                match self
                    .list
                    .binary_search_by_key(&days_from_start, |(d, _v)| *d)
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

impl ResourceSummaryModel {
    fn form_model_point(&self, game_data: &ModelDataPoint) -> Option<(u64, Vec<f64>)> {
        if game_data.campaign_name != self.spec.campaign_name {
            None
        } else if let Some(empire) = game_data
            .empires
            .iter()
            .find(|e| e.name == self.spec.empire)
        {
            let days_from_start = game_data
                .date
                .signed_duration_since(NaiveDate::from_ymd(2200, 01, 01))
                .num_days() as u64;
            Some((days_from_start, self.get_resource_values(empire)))
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

impl ModelSpec for ResourceSummaryModelSpec {
    type Model = ResourceSummaryModel;
}

#[cfg(test)]
mod tests {}
