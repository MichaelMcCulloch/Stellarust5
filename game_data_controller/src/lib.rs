use std::{hash::BuildHasherDefault, path::Path, sync::Arc};

use actix_broadcaster::{ActixBroadcaster, Broadcaster, Client};
use crossbeam::{
    channel::{unbounded, Receiver},
    thread::Scope,
};
use dashmap::{mapref::entry::Entry, DashMap};
use directory_watcher::{create_directory_watcher_and_scan_root, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use fxhash::{FxBuildHasher, FxHasher};
use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use scan_root::ScanAllFoldersAndFiles;
mod filter;
mod scan_root;
pub struct GameModelController {
    broadcasters_map:
        Arc<DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>>,
    game_data_history: Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
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
        let game_data_history = Arc::new(DashMap::with_hasher(FxBuildHasher::default()));
        let broadcasters_map = Arc::new(DashMap::with_hasher(FxBuildHasher::default()));
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
    pub fn get_client(&self, model_spec_enum: ModelSpecEnum) -> Option<Client> {
        match self.broadcasters_map.entry(model_spec_enum) {
            Entry::Occupied(entry) => {
                let (model, broadcaster) = entry.get();

                Some(broadcaster.new_client_with_message(&model.get()))
            }
            Entry::Vacant(entry) => {
                let (mut model, broadcaster) = (
                    ModelEnum::create(entry.key().clone()),
                    ActixBroadcaster::create(),
                );
                match model.update_all(&self.game_data_history.clone()) {
                    Some(message) => {
                        let client = broadcaster.new_client_with_message(&message);
                        entry.insert((model, broadcaster));
                        return Some(client);
                    }
                    None => return None,
                }
            }
        }
    }

    /// * `model_data` - new data point
    /// * `model_history` - Existing data
    fn reconcile(
        model_data: &ModelDataPoint,
        game_data_history: &Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
    ) {
        match game_data_history.entry(model_data.campaign_name.clone()) {
            Entry::Occupied(mut entry) => {
                match entry
                    .get()
                    .binary_search_by_key(&model_data.date, |m| m.date)
                {
                    Ok(_index) => {}
                    Err(pos) => entry.get_mut().insert(pos, model_data.clone()),
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![model_data.clone()]);
            }
        }
    }
    /// Spawns the event loop in the scope
    /// * `scope` - Crossbeam scope
    /// * `info_struct_receiver` - Receiving end of the sender embeded in the directory watcher
    /// * `model_history` - Arc to the lock for reconciling new data with existing data
    /// * `broadcasters_map` - Map of receivers indexed by the request they made
    fn spawn_event_loop(
        scope: &Scope,
        info_struct_receiver: Receiver<ModelDataPoint>,
        game_data_history: Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
        broadcasters_map: Arc<
            DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>,
        >,
    ) {
        scope.spawn(move |_s| loop {
            match info_struct_receiver.recv() {
                Ok(data_point) => {
                    Self::reconcile(&data_point, &game_data_history);
                    Self::broadcast_model_changes(&broadcasters_map, &data_point);
                }
                Err(_) => break,
            };
        });
    }
    fn broadcast_model_changes(
        broadcasters_map: &Arc<
            DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>,
        >,
        data_point: &ModelDataPoint,
    ) {
        broadcasters_map.retain(|_, (model, broadcaster)| match model.update(data_point) {
            Some(output) => {
                let recipients = broadcaster.send(&output);
                recipients != 0
            }
            None => true,
        })
    }
}
