use super::campaign_list::Empire;
use crate::{Model, ModelSpec};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::{model::ModelDataPoint, player::PlayerClass};
use serde_derive::Serialize;
use std::{collections::HashSet, hash::BuildHasherDefault};

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct EmpireListModelSpec {
    pub campaign_name: String,
}

#[derive(Serialize, Debug)]
pub struct EmpireListModel {
    set: HashSet<Empire>,
    spec: EmpireListModelSpec,
}
impl Model for EmpireListModel {
    type ModelSpec = EmpireListModelSpec;
    type Representation = Vec<Empire>;

    fn create(spec: Self::ModelSpec) -> Self {
        Self {
            set: HashSet::new(),
            spec,
        }
    }

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        if game_data.campaign_name == self.spec.campaign_name {
            self.set.extend(
                game_data
                    .empires
                    .iter()
                    .map(|e| Empire {
                        name: e.name.clone(),
                        player: match &e.driver {
                            PlayerClass::Human(name) => Some(name.clone()),
                            PlayerClass::Computer => None,
                        },
                    })
                    .collect::<Vec<_>>(),
            );
            Some(self.set.clone().into_iter().collect())
        } else {
            None
        }
    }

    fn update_all(
        &mut self,
        game_data_history: &DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>,
    ) -> Option<Self::Representation> {
        match game_data_history.get(&self.spec.campaign_name) {
            Some(vec) => {
                self.set = vec.iter().fold(HashSet::new(), |mut acc, game_data| {
                    let empires = game_data
                        .empires
                        .iter()
                        .map(|e| Empire {
                            name: e.name.clone(),
                            player: match &e.driver {
                                PlayerClass::Human(name) => Some(name.clone()),
                                PlayerClass::Computer => None,
                            },
                        })
                        .collect::<Vec<_>>();
                    acc.extend(empires);
                    acc
                });

                Some(self.set.clone().into_iter().collect())
            }
            None => None,
        }
    }

    fn get(&self) -> Self::Representation {
        self.set.clone().into_iter().collect()
    }
}

impl ModelSpec for EmpireListModelSpec {
    type Model = EmpireListModel;
}
