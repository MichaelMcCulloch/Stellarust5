use crate::filter;
use crate::scan_root;
use actix_broadcaster::{ActixBroadcaster, Broadcaster};
use crossbeam::{
    channel::{Receiver, Sender},
    thread::Scope,
};
use dashmap::{mapref::entry::Entry, DashMap};
use directory_watcher::{DefaultWatcher, DirectoryWatcher, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use fxhash::FxHasher;
use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use rusqlite::Connection;
use scan_root::ScanAllFoldersAndFiles;
use std::hash::BuildHasherDefault;
use std::path::Path;
use std::sync::Arc;

pub(crate) fn get_directory_watcher(
    info_struct_sender: Sender<ModelDataPoint>,
    game_directory: &Path,
) -> RecommendedWatcher {
    DefaultWatcher::create_directory_watcher_and_scan_root(
        CloseWriteFilter,
        EndsWithSavFilter,
        GameDataInfoStructReader,
        move |message: ModelDataPoint| -> () {
            log::trace!("Discovered {:?} -- {}", message.date, message.campaign_name);

            info_struct_sender.send(message).unwrap();
        },
        ScanAllFoldersAndFiles,
        &game_directory,
        RecursiveMode::Recursive,
    )
}
pub(crate) fn query_models(db_connection: &Connection) -> anyhow::Result<Vec<ModelDataPoint>> {
    let sql_select_table_names =
        &"SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';";
    let sql_create_table_data_points = &"CREATE TABLE data_points (
            blob    TEXT NOT NULL
        );";
    let sql_select_all_data = "SELECT * FROM data_points;";

    let tables = db_connection
        .prepare(sql_select_table_names)?
        .query_map([], |row| row.get::<_, String>(0))?
        .filter_map(|s| s.ok())
        .collect::<Vec<_>>();
    if !tables.contains(&"data_points".to_string()) {
        db_connection.execute(sql_create_table_data_points, [])?;
        log::info!("Populating empty database `stellarust_model_history.db` in game folder");
        Ok(vec![])
    } else {
        let extant_data = db_connection
            .prepare(sql_select_all_data)?
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|s| s.ok())
            .filter_map(|s| serde_json::from_str(s.as_str()).ok())
            .map(|model: ModelDataPoint| {
                log::trace!("Discovered {:?} -- {}", model.date, model.campaign_name);
                model
            })
            .collect::<Vec<ModelDataPoint>>();

        Ok(extant_data)
    }
}
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
/// Spawns the event loop in the scope
/// * `scope` - Crossbeam scope
/// * `info_struct_receiver` - Receiving end of the sender embeded in the directory watcher
/// * `model_history` - Arc to the lock for reconciling new data with existing data
/// * `broadcasters_map` - Map of receivers indexed by the request they made

pub(crate) fn write_to_db(data_point: &ModelDataPoint, db_connection: &Connection) {
    let msg = serde_json::to_string(data_point).unwrap();
    db_connection
        .execute("INSERT INTO data_points (blob) values (?)", [msg])
        .unwrap();
}
pub(crate) fn broadcast_model_changes(
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

/// * `model_data` - new data point
/// * `model_history` - Existing data
pub(crate) fn reconcile(
    data_point: &ModelDataPoint,
    game_data_history: &Arc<DashMap<String, Vec<ModelDataPoint>, BuildHasherDefault<FxHasher>>>,
) -> bool {
    match game_data_history.entry(data_point.campaign_name.clone()) {
        Entry::Occupied(mut entry) => {
            match entry
                .get()
                .binary_search_by_key(&data_point.date, |m| m.date)
            {
                Ok(_index) => {
                    // log::warn!(
                    //     "Tried to insert duplicate entry for campaign key {}, entries are {}",
                    //     data_point.campaign_name,
                    //     match entry.get().get(_index).unwrap() == data_point {
                    //         true => "the same",
                    //         false => "different",
                    //     }
                    // );
                    false
                }
                Err(pos) => {
                    entry.get_mut().insert(pos, data_point.clone());
                    true
                }
            }
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![data_point.clone()]);
            true
        }
    }
}
