use std::{collections::HashMap, vec};

use crate::{Model, ModelSpec};
use game_data_info_struct::{ModelDataPoint, PlayerClass};
use serde_derive::Serialize;

use super::campaign_list::Empire;

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct EmpireListModelSpec {
    pub campaign_name: String,
}

#[derive(Serialize, Debug)]
pub struct EmpireListModel {
    list: Vec<Empire>,
    spec: EmpireListModelSpec,
}
impl Model for EmpireListModel {
    type ModelSpec = EmpireListModelSpec;

    fn create(spec: Self::ModelSpec) -> Self {
        Self { list: vec![], spec }
    }

    type Representation = Vec<Empire>;

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        if game_data.campaign_name == self.spec.campaign_name {
            self.list = game_data
                .empires
                .iter()
                .map(|e| Empire {
                    name: e.name.clone(),
                    player: match &e.driver {
                        PlayerClass::Human(name) => Some(name.clone()),
                        PlayerClass::Computer => None,
                    },
                })
                .collect();

            Some(self.list.clone())
        } else {
            None
        }
    }

    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation> {
        match game_data_history.get(&self.spec.campaign_name) {
            Some(vec) => {
                self.list = vec
                    .last()
                    .unwrap()
                    .empires
                    .iter()
                    .map(|e| Empire {
                        name: e.name.clone(),
                        player: match &e.driver {
                            PlayerClass::Human(name) => Some(name.clone()),
                            PlayerClass::Computer => None,
                        },
                    })
                    .collect();

                Some(self.list.clone())
            }
            None => None,
        }
    }

    fn get(&self) -> Self::Representation {
        self.list.clone()
    }
}

impl ModelSpec for EmpireListModelSpec {
    type Model = EmpireListModel;
}
