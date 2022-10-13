use std::hash::{BuildHasherDefault, Hash};

use crate::{Model, ModelSpec};
use chrono::NaiveDate;
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{
    empire::EmpireData, index::Index, model::ModelDataPoint, resource::ResourceClass,
};
use serde::{ser::SerializeSeq, Serialize};
use serde_derive::Serialize;
use stellarust::{START_OF_GAME_DATE, START_OF_GAME_MONTH, START_OF_GAME_YEAR};

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct ResourceSummaryModelSpec {
    pub resources: Vec<ResourceClass>,
    pub empire: String,
    pub campaign_name: String,
}

#[derive(PartialEq, Clone, Debug)]
struct LineChartDataPoint {
    days_since_start: u64,
    resource_map: Vec<f64>,
}

impl Serialize for LineChartDataPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.resource_map.len() + 1))?;
        seq.serialize_element(&self.days_since_start)?;
        for res in self.resource_map.iter() {
            seq.serialize_element(res)?;
        }
        seq.end()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LineChartData(Vec<LineChartDataPoint>);

impl Serialize for LineChartData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for res in self.0.iter() {
            seq.serialize_element(res)?;
        }
        seq.end()
    }
}

/// ResourceSummary
/// Intended to be displayable with a `LineChart`
/// Representation = `List<(Days since 2200.01.01, Resource Values)>`
#[derive(Debug)]
pub struct ResourceSummaryModel {
    line_chart_data: LineChartData,
    spec: ResourceSummaryModelSpec,
}
impl Model for ResourceSummaryModel {
    type ModelSpec = ResourceSummaryModelSpec;
    type Representation = LineChartData; // This is not very readable. TODO: Instead let's wrap it in a struct, then employ serde::serialize_with to turn a code acceptable copy into the representation google charts needs

    fn create(spec: Self::ModelSpec) -> Self {
        Self {
            line_chart_data: LineChartData(vec![]),
            spec,
        }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        match self.form_model_point(game_data) {
            Some(data) => {
                match self
                    .line_chart_data
                    .0
                    .binary_search_by_key(&data.days_since_start, |datapoint| {
                        datapoint.days_since_start
                    }) {
                    Ok(index) => {
                        self.line_chart_data.0.remove(index);
                        self.line_chart_data.0.insert(index, data.clone());
                    }
                    Err(index) => {
                        self.line_chart_data.0.insert(index, data.clone());
                    }
                }
                Some(LineChartData(vec![data]))
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

                Some(self.line_chart_data.clone())
            }
            None => None,
        }
    }

    fn get(&self) -> Self::Representation {
        self.line_chart_data.clone()
    }
}

impl ResourceSummaryModel {
    fn form_model_point(&self, game_data: &ModelDataPoint) -> Option<LineChartDataPoint> {
        if game_data.campaign_name != self.spec.campaign_name {
            None
        } else if let Some(empire) = game_data
            .empires
            .iter()
            .find(|e| e.name == self.spec.empire)
        {
            let days_since_start = game_data
                .date
                .signed_duration_since(NaiveDate::from_ymd(
                    START_OF_GAME_YEAR,
                    START_OF_GAME_MONTH,
                    START_OF_GAME_DATE,
                ))
                .num_days() as u64;
            Some(LineChartDataPoint {
                days_since_start,
                resource_map: self.get_resource_values(empire),
            })
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
