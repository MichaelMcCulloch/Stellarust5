use std::{hash::BuildHasherDefault, sync::Arc};

use actix_broadcaster::ActixBroadcaster;
use crossbeam::{channel::Receiver, thread::Scope};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct_reader::ModelDataPoint;
use model_info_struct::enums::{ModelEnum, ModelSpecEnum};

pub trait EventLoop {
    fn spawn_event_loop(
        scope: &Scope,
        info_struct_receiver: Receiver<ModelDataPoint>,
        game_data_history: Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
        broadcasters_map: Arc<
            DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>,
        >,
    );
}
