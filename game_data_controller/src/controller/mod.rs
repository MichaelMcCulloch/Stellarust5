use crate::filter;
use crate::scan_root;
use actix_broadcaster::{ActixBroadcaster, Broadcaster, Client};
use crossbeam::{
    channel::{Receiver, Sender},
    thread::Scope,
};
use dashmap::{mapref::entry::Entry, DashMap};
use directory_watcher::{DefaultWatcher, DirectoryWatcher, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use fxhash::{FxBuildHasher, FxHasher};
use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use scan_root::ScanAllFoldersAndFiles;
use std::hash::BuildHasherDefault;
use std::path::Path;
use std::sync::Arc;

pub struct GameModelController {
    broadcasters_map:
        Arc<DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>>,
    game_data_history: Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
    _watcher: RecommendedWatcher,
}
impl GameModelController {
    fn new(watcher: RecommendedWatcher) -> GameModelController {
        let game_model_controller = Self {
            broadcasters_map: Arc::new(DashMap::with_hasher(FxBuildHasher::default())),
            game_data_history: Arc::new(DashMap::with_hasher(FxBuildHasher::default())),
            _watcher: watcher,
        };
        game_model_controller
    }

    pub fn create(
        game_directory: &Path,
        scope: &Scope<'_>,
        info_struct_channel: (Sender<ModelDataPoint>, Receiver<ModelDataPoint>),
    ) -> Self {
        let watcher = DefaultWatcher::create_directory_watcher_and_scan_root(
            CloseWriteFilter,
            EndsWithSavFilter,
            GameDataInfoStructReader,
            move |message| -> () {
                info_struct_channel.0.send(message).unwrap();
            },
            ScanAllFoldersAndFiles,
            &game_directory,
            RecursiveMode::Recursive,
        );

        let game_model_controller = GameModelController::new(watcher);
        // if a database does not exist, create one
        // read all history from the database
        // verify contents of folder match contents of db
        game_model_controller.spawn_event_loop(scope, info_struct_channel.1);
        game_model_controller
    }

    /// * `model_data` - new data point
    /// * `model_history` - Existing data
    fn reconcile(
        data_point: &ModelDataPoint,
        game_data_history: &Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
    ) {
        match game_data_history.entry(data_point.campaign_name.clone()) {
            Entry::Occupied(mut entry) => {
                match entry
                    .get()
                    .binary_search_by_key(&data_point.date, |m| m.date)
                {
                    Ok(index) => {
                        log::warn!(
                            "Tried to insert duplicate entry for campaign key {}, entries are {}",
                            data_point.campaign_name,
                            match entry.get().get(index).unwrap() == data_point {
                                true => "the same",
                                false => "different",
                            }
                        )
                    }
                    Err(pos) => entry.get_mut().insert(pos, data_point.clone()),
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![data_point.clone()]);
            }
        }
    }
    /// Spawns the event loop in the scope
    /// * `scope` - Crossbeam scope
    /// * `info_struct_receiver` - Receiving end of the sender embeded in the directory watcher
    /// * `model_history` - Arc to the lock for reconciling new data with existing data
    /// * `broadcasters_map` - Map of receivers indexed by the request they made
    fn spawn_event_loop(&self, scope: &Scope, info_struct_receiver: Receiver<ModelDataPoint>) {
        let game_data_history = self.game_data_history.clone();
        let broadcasters_map = self.broadcasters_map.clone();
        scope.spawn(move |_s| loop {
            match info_struct_receiver.recv() {
                Ok(data_point) => {
                    Self::reconcile(&data_point, &game_data_history);

                    if !broadcasters_map.is_empty() {
                        Self::broadcast_model_changes(&broadcasters_map, &data_point);
                    }
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
        broadcasters_map.retain(
            |spec, (model, broadcaster)| match model.update(data_point) {
                Some(output) => {
                    let recipients = broadcaster.send(&output);

                    log::trace!("Broadcast {:?}: Retaining {} clients", spec, recipients);
                    recipients != 0
                }
                None => true,
            },
        );
        log::trace!("Broadcast: Retaining {} clients", broadcasters_map.len());
    }
    pub fn get_client(&self, model_spec_enum: ModelSpecEnum) -> Option<Client> {
        match self.broadcasters_map.entry(model_spec_enum.clone()) {
            Entry::Vacant(entry) => {
                let mut model = ModelEnum::create(entry.key().clone());

                match model.update_all(&self.game_data_history.clone()) {
                    Some(message) => {
                        let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

                        let broadcaster = ActixBroadcaster::create(sender);
                        let client = broadcaster.new_client_with_message(&message);

                        entry.insert((model, broadcaster));

                        let broadcasters_map = self.broadcasters_map.clone();
                        actix_rt::spawn(async move {
                            match receiver.recv().await.unwrap() {
                                () => {
                                    log::info!("Removing entry for {:?}", model_spec_enum);
                                    broadcasters_map.remove(&model_spec_enum)
                                }
                            }
                        });

                        Some(client)
                    }
                    None => None,
                }
            }
            Entry::Occupied(entry) => {
                let (model, broadcaster) = entry.get();

                Some(broadcaster.new_client_with_message(&model.get()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, time::Duration};

    use chrono::NaiveDate;
    use crossbeam::{channel::unbounded, thread};
    use model_info_struct::model::campaign_list::CampaignListModelSpec;
    use stellarust::PROD_TEST_EMPTY_FOLDER;

    use super::*;
    #[actix_rt::test]
    async fn game_data_controller_behavior_tests() {
        thread::scope(|scope| {
            std::env::set_var("RUST_LOG", "info");
            env_logger::init();
            let (tx, rx) = unbounded();
            let c = GameModelController::create(
                &PathBuf::from(PROD_TEST_EMPTY_FOLDER),
                scope,
                (tx.clone(), rx),
            );
            {
                std::thread::sleep(Duration::from_millis(5));

                assert!(c.broadcasters_map.clone().is_empty());
                assert!(c.game_data_history.clone().is_empty());
                log::info!("Empty On Startup:: Passed");
            }
            let client = {
                let client = c.get_client(ModelSpecEnum::CampaignList(CampaignListModelSpec));
                assert_eq!(c.broadcasters_map.clone().len(), 1);
                log::info!(
                    "Broadcasters populated with one key after requesting the client:: Passed"
                );
                client
            };
            {
                tx.send(ModelDataPoint {
                    campaign_name: "TEST_CAMPAIGN".to_string(),
                    date: NaiveDate::MAX.into(),
                    empires: vec![],
                })
                .unwrap();
                std::thread::sleep(Duration::from_millis(5));

                assert_eq!(c.game_data_history.clone().len(), 1);
                log::info!("Game history populated with one key after pushing a model:: Passed");
            }
            {
                drop(client);
                //wait for remove client
                std::thread::sleep(Duration::from_millis(5));

                tx.send(ModelDataPoint {
                    campaign_name: "TEST_CAMPAIGN".to_string(),
                    date: NaiveDate::MIN.into(),
                    empires: vec![],
                })
                .unwrap();
                std::thread::sleep(Duration::from_millis(5));

                assert!(c.broadcasters_map.clone().is_empty());
                log::info!("Broadcasters Empty if the last client drops:: Passed");
            }
        })
        .unwrap();
    }
}
