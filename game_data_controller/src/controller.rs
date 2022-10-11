use actix_broadcaster::{ActixBroadcaster, Broadcaster, Client};
use crossbeam::{
    channel::{Receiver, Sender},
    thread::Scope,
};
use dashmap::{mapref::entry::Entry, DashMap};
use fxhash::{FxBuildHasher, FxHasher};
use game_data_info_struct_reader::ModelDataPoint;
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use rusqlite::Connection;
use std::hash::BuildHasherDefault;
use std::path::Path;
use std::sync::Arc;

use crate::functions::*;

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

    /// - Create a directory watcher
    /// - Open the connection to the db
    /// - Process data from the directory watcher
    /// - Process data from the db connection
    /// - Start the event loop
    pub fn create(
        game_directory: &Path,
        scope: &Scope<'_>,
        info_struct_channel: (Sender<ModelDataPoint>, Receiver<ModelDataPoint>),
    ) -> Self {
        let (info_struct_sender, info_struct_receiver) = info_struct_channel;

        let watcher = get_directory_watcher(info_struct_sender, game_directory);

        let db_connection = Connection::open({
            let mut game_directory = game_directory.to_path_buf();
            game_directory.push("stellarust_model_history.db");
            game_directory
        })
        .unwrap();
        let extant_data = query_models(&db_connection).unwrap();
        log::info!(
            "Discovered {} files from game folder",
            info_struct_receiver.len()
        );
        log::info!(
            "Discovered {} data points from `stellarust_model_history.db` in game folder",
            extant_data.len()
        );
        let game_model_controller = GameModelController::new(watcher);

        let game_data_history = game_model_controller.game_data_history.clone();
        let broadcasters_map = game_model_controller.broadcasters_map.clone();

        for data in extant_data {
            reconcile(&data, &game_data_history);
        }

        spawn_event_loop(
            scope,
            game_data_history,
            broadcasters_map,
            info_struct_receiver,
            db_connection,
        );

        game_model_controller
    }

    /// Obtain a client for the model spec
    /// - If an entry for the spec does not exist:
    ///     - populate a model
    ///     - populate a broadcaster
    ///     - get a client for the broadcaster
    ///     - Send the model data to the client
    ///     - Save the pair
    /// - Else
    ///     - Get the model & broadcaster pair
    ///     - get a client for the broadcaster
    ///     - Send the model data to the client
    ///
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
                                    log::trace!("Removing entry for {:?}", model_spec_enum);
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
            {
                let db = {
                    let mut path = PathBuf::from(PROD_TEST_EMPTY_FOLDER);
                    path.push("stellarust_model_history.db");
                    path
                };

                match std::fs::remove_file(db) {
                    _ => {}
                };
            }
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
                std::thread::sleep(Duration::from_millis(30));

                tx.send(ModelDataPoint {
                    campaign_name: "TEST_CAMPAIGN".to_string(),
                    date: NaiveDate::MIN.into(),
                    empires: vec![],
                })
                .unwrap();
                std::thread::sleep(Duration::from_millis(30));

                assert!(c.broadcasters_map.clone().is_empty());
                log::info!("Broadcasters Empty if the last client drops:: Passed");
            }
        })
        .unwrap();
    }
}
