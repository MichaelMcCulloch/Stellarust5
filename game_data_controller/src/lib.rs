use std::collections::HashMap;

use actix_broadcaster::{Broadcaster, Client};
use game_data_info_struct::ModelDataPoint;
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};

pub struct GameModelController {
    broadcasters_map: HashMap<ModelSpecEnum, (ModelEnum, Broadcaster)>,
    game_data_history: Vec<ModelDataPoint>,
}

impl GameModelController {
    pub fn create() -> Self {
        Self {
            broadcasters_map: HashMap::new(),
            game_data_history: vec![],
        }
    }

    pub fn getClient(&mut self, model_spec_enum: ModelSpecEnum) -> Client {
        let (model, broadcaster) = self
            .broadcasters_map
            .entry(model_spec_enum.clone())
            .or_insert((ModelEnum::create(model_spec_enum), Broadcaster::create()));
        let message = model.update_all(&self.game_data_history);
        let client = broadcaster.new_client_with_message(&message);
        client
    }
}
