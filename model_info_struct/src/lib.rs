use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct::model::ModelDataPoint;
use serde::Serialize;
use std::hash::{BuildHasherDefault, Hash};

pub mod model;

pub mod enums;

pub use game_data_info_struct::{
    resource::ResourceClass, resource::ResourceCode, resource::ALL_RESOURCES,
};
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
        game_data_history: &DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>,
    ) -> Option<Self::Representation>;
    fn get(&self) -> Self::Representation;
}
