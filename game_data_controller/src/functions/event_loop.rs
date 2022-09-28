use actix_broadcaster::ActixBroadcaster;
use crossbeam::{channel::Receiver, thread::Scope};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct_reader::ModelDataPoint;
use model_info_struct::enums::{ModelEnum, ModelSpecEnum};
use rusqlite::Connection;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

use super::broadcast_model_changes;
use super::reconcile;
use super::write_to_db;

/// Spawns the event loop in the scope
/// * `scope` - Crossbeam scope
/// * `info_struct_receiver` - Receiving end of the sender embeded in the directory watcher
/// * `model_history` - Arc to the lock for reconciling new data with existing data
/// * `broadcasters_map` - Map of receivers indexed by the request they made

pub(crate) fn spawn_event_loop(
    scope: &Scope,
    game_data_history: Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
    broadcasters_map: Arc<
        DashMap<ModelSpecEnum, (ModelEnum, ActixBroadcaster), BuildHasherDefault<FxHasher>>,
    >,
    info_struct_receiver: Receiver<ModelDataPoint>,
    db_connection: Connection,
) {
    scope.spawn(move |_s| loop {
        match info_struct_receiver.recv() {
            Ok(data_point) => {
                let new = reconcile(&data_point, &game_data_history);

                if !broadcasters_map.is_empty() {
                    broadcast_model_changes(&broadcasters_map, &data_point);
                }
                if new {
                    write_to_db(&data_point, &db_connection)
                }
            }
            Err(_) => break,
        };
    });
}