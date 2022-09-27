use std::hash::BuildHasherDefault;

use crate::{Model, ModelSpec};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{
    date::Date, BudgetMapIndex, EmpireData, ModelDataPoint, ResourceClass,
};
use serde_derive::Serialize;

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

impl BudgetStreamGraphModel {
    fn form_model_point(&self, game_data: &ModelDataPoint) -> Option<(Date, Vec<f64>)> {
        if game_data.campaign_name != self.spec.campaign_name {
            None
        } else {
            if let Some(empire) = game_data
                .empires
                .iter()
                .find(|e| e.name == self.spec.empire)
            {
                Some((Date::from(game_data.date), self.get_budget_values(empire)))
            } else {
                None
            }
        }
    }
    fn get_budget_values(&self, empire_data: &EmpireData) -> Vec<f64> {
        let mut ret = vec![];
        for resource in self.spec.resources.iter() {
            ret.push(
                empire_data
                    .budget
                    .income
                    .get(resource.index())
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

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;
    use fxhash::FxBuildHasher;
    use game_data_info_struct::{Budget, PlayerClass, Resources};

    const VAL: Vec<(String, f64)> = vec![];
    use super::*;
    #[test]
    fn update_given_a_model_spec_and_a_single_data_point_a_model_constructed_with_that_spec_and_given_that_data_point_returns_a_single_element_representing_that_data_point(
    ) {
        let campaign_name = "CAMPAIGN_NAME".to_string();
        let empire = "EMPIRE_NAME".to_string();
        let resources = vec![
            ResourceClass::Energy,
            ResourceClass::Alloys,
            ResourceClass::Minerals,
        ];

        let spec = BudgetStreamGraphModelSpec {
            campaign_name: campaign_name.clone(),
            empire: empire.clone(),
            resources,
        };

        let date = NaiveDate::from_ymd(2200, 01, 01);
        let driver = PlayerClass::Human("HUMAN".to_string());
        let mut balance = [VAL; 17];
        balance[ResourceClass::Energy.index()] = vec![("ALL".to_string(), 100f64)];
        balance[ResourceClass::Alloys.index()] = vec![("ALL".to_string(), 50f64)];
        balance[ResourceClass::Minerals.index()] = vec![("ALL".to_string(), 25f64)];

        let budget = Budget {
            income: [VAL; 17],
            expense: [VAL; 17],
        };

        let resources = Resources {
            energy: 0f64,
            minerals: 0f64,
            food: 0f64,
            physics_research: 0f64,
            society_research: 0f64,
            engineering_research: 0f64,
            influence: 0f64,
            unity: 0f64,
            consumer_goods: 0f64,
            alloys: 0f64,
            volatile_motes: 0f64,
            exotic_gases: 0f64,
            rare_crystals: 0f64,
            sr_living_metal: 0f64,
            sr_zro: 0f64,
            sr_dark_matter: 0f64,
        };

        let model_data_point = ModelDataPoint {
            campaign_name,
            date,
            empires: vec![EmpireData {
                name: empire,
                driver,
                budget,
                resources,
            }],
        };
        let mut model = BudgetStreamGraphModel::create(spec);

        match model.update(&model_data_point) {
            Some(v) => assert_eq!(v, vec![(Date::from(date), vec![100f64, 50f64, 25f64])]),
            None => assert!(false, "Failed to return a value!!!"),
        };
    }
    #[test]
    fn update_all_given_a_model_spec_and_an_unsorted_list_of_model_specs_containing_duplicates_a_model_constructed_with_that_spec_and_given_that_data_point_returns_a_sorted_list_of_uniquely_dated_representation_elements(
    ) {
        let campaign_name = "CAMPAIGN_NAME".to_string();
        let empire = "EMPIRE_NAME".to_string();
        let resources = vec![ResourceClass::Energy];

        let spec = BudgetStreamGraphModelSpec {
            campaign_name: campaign_name.clone(),
            empire: empire.clone(),
            resources,
        };

        let driver = PlayerClass::Human("HUMAN".to_string());
        let mut balance = [VAL; 17];
        balance[ResourceClass::Energy.index()] = vec![("ALL".to_string(), 100f64)];

        let budget = Budget {
            income: [VAL; 17],
            expense: [VAL; 17],
        };

        let resources = Resources {
            energy: 0f64,
            minerals: 0f64,
            food: 0f64,
            physics_research: 0f64,
            society_research: 0f64,
            engineering_research: 0f64,
            influence: 0f64,
            unity: 0f64,
            consumer_goods: 0f64,
            alloys: 0f64,
            volatile_motes: 0f64,
            exotic_gases: 0f64,
            rare_crystals: 0f64,
            sr_living_metal: 0f64,
            sr_zro: 0f64,
            sr_dark_matter: 0f64,
        };

        let date_1 = NaiveDate::from_ymd(2201, 01, 01);
        let date_2 = NaiveDate::from_ymd(2202, 01, 01);
        let date_3 = NaiveDate::from_ymd(2203, 01, 01);
        let model_data_point_1 = ModelDataPoint {
            campaign_name: campaign_name.clone(),
            date: date_1,
            empires: vec![EmpireData {
                name: empire.clone(),
                driver: driver.clone(),
                budget: budget.clone(),
                resources: resources.clone(),
            }],
        };
        let model_data_point_2 = ModelDataPoint {
            campaign_name: campaign_name.clone(),
            date: date_2,
            empires: vec![EmpireData {
                name: empire.clone(),
                driver: driver.clone(),
                budget: budget.clone(),
                resources: resources.clone(),
            }],
        };
        let model_data_point_3 = ModelDataPoint {
            campaign_name: campaign_name.clone(),
            date: date_3,
            empires: vec![EmpireData {
                name: empire.clone(),
                driver: driver.clone(),
                budget: budget.clone(),
                resources: resources.clone(),
            }],
        };
        let game_data_history = DashMap::with_hasher(FxBuildHasher::default());
        game_data_history.insert(
            campaign_name.clone(),
            vec![
                model_data_point_1,
                model_data_point_3,
                model_data_point_2.clone(),
                model_data_point_2,
            ],
        );

        let mut model = BudgetStreamGraphModel::create(spec);

        match model.update_all(&game_data_history) {
            Some(v) => assert_eq!(
                v,
                vec![
                    (Date::from(date_1), vec![100f64]),
                    (Date::from(date_2), vec![100f64]),
                    (Date::from(date_3), vec![100f64]),
                ]
            ),
            None => assert!(false, "Failed to return a value!!!"),
        };
    }
}
