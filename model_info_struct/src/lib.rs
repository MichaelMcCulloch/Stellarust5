use std::{collections::HashMap, hash::Hash};

use game_data_info_struct::ModelDataPoint;
use serde::Serialize;

pub mod model;

pub mod enums;

pub use game_data_info_struct::ResourceClass;
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
