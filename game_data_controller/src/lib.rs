use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, RwLock},
};

use actix_broadcaster::{Broadcaster, Client};
use crossbeam::{
    channel::{unbounded, Receiver},
    thread::Scope,
};
use directory_watcher::{create_directory_watcher_and_scan_root, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use game_data_info_struct::ModelDataPoint;
use game_data_info_struct_reader::GameDataInfoStructReader;
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use rayon::prelude::{ParallelDrainFull, ParallelIterator};
use scan_root::ScanAllFoldersAndFiles;
mod filter;
mod scan_root;
pub struct GameModelController {
    broadcasters_map: Arc<RwLock<HashMap<ModelSpecEnum, (ModelEnum, Broadcaster)>>>,
    game_data_history: Arc<RwLock<HashMap<String, Vec<ModelDataPoint>>>>,
    _watcher: RecommendedWatcher,
}

impl GameModelController {
    pub fn create(game_directory: &Path, scope: &Scope<'_>) -> Self {
        let (info_struct_sender, info_struct_receiver) = unbounded();
        let watcher = create_directory_watcher_and_scan_root(
            CloseWriteFilter,
            EndsWithSavFilter,
            GameDataInfoStructReader,
            move |message| -> () {
                info_struct_sender.clone().send(message).unwrap();
            },
            ScanAllFoldersAndFiles,
            &game_directory,
            RecursiveMode::Recursive,
        );
        let game_data_history = Arc::new(RwLock::new(HashMap::new()));
        let broadcasters_map = Arc::new(RwLock::new(HashMap::new()));
        let game_model_controller = Self {
            broadcasters_map: broadcasters_map.clone(),
            game_data_history: game_data_history.clone(),
            _watcher: watcher,
        };
        Self::spawn_event_loop(
            scope,
            info_struct_receiver,
            game_data_history,
            broadcasters_map,
        );

        game_model_controller
    }

    /// 1. Populates the map of ModelRequests to the (Model, Broadcaster) pairs
    /// 2. populates a new model based on the parameters of the spec
    /// 3. populates a client from the broadcaster and sends that client the model it asked for
    /// 4. returns that client
    pub fn get_client(&mut self, model_spec_enum: ModelSpecEnum) -> Client {
        let mut broadcaster_map = self.broadcasters_map.write().unwrap();

        let (model, broadcaster) = broadcaster_map
            .entry(model_spec_enum.clone())
            .or_insert((ModelEnum::create(model_spec_enum), Broadcaster::create()));

        let game_data_history = self.game_data_history.read().unwrap();
        let message = model.update_all(&game_data_history.clone());
        let client = broadcaster.new_client_with_message(&message);
        client
    }

    /// * `model_data` - new data point
    /// * `model_history` - Existing data
    fn reconcile(
        model_data: &ModelDataPoint,
        model_history: &Arc<RwLock<HashMap<String, Vec<ModelDataPoint>>>>,
    ) {
        model_history
            .write()
            .unwrap()
            .entry(model_data.campaign_name.clone())
            .or_insert(vec![])
            .push(model_data.clone());
    }
    /// Spawns the event loop in the scope
    /// * `scope` - Crossbeam scope
    /// * `info_struct_receiver` - Receiving end of the sender embeded in the directory watcher
    /// * `model_history` - Arc to the lock for reconciling new data with existing data
    /// * `broadcasters_map` - Map of receivers indexed by the request they made
    fn spawn_event_loop(
        scope: &Scope,
        info_struct_receiver: Receiver<ModelDataPoint>,
        model_history: Arc<RwLock<HashMap<String, Vec<ModelDataPoint>>>>,
        broadcasters_map: Arc<RwLock<HashMap<ModelSpecEnum, (ModelEnum, Broadcaster)>>>,
    ) {
        scope.spawn(move |_s| loop {
            match info_struct_receiver.recv() {
                Ok(data_point) => {
                    Self::reconcile(&data_point, &model_history);
                    let mut guard = broadcasters_map.write().unwrap();
                    let mut map = std::mem::take(&mut *guard);

                    // The reason this is so ugly is because we are required to mutate the model and broad caster, and we can't do that if they are behind a mutable reference
                    *guard = map
                        .par_drain()
                        .fold(
                            || HashMap::new(),
                            |mut a, (_spec, (mut model, broadcaster))| match model
                                .update(&data_point)
                            {
                                Some(output) => {
                                    if broadcaster.send(&output) {
                                        a.insert(_spec, (model, broadcaster));
                                        a
                                    } else {
                                        a // Only remove the broadcaster if the broadcaster says there are no clients left
                                    }
                                }
                                None => {
                                    a.insert(_spec, (model, broadcaster));
                                    a
                                }
                            },
                        )
                        .reduce(
                            || HashMap::new(),
                            |mut a, b| {
                                a.extend(b.into_iter());
                                a
                            },
                        );
                }
                Err(_) => break,
            };
        });
    }
}
