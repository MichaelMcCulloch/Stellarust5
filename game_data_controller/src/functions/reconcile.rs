use dashmap::{mapref::entry::Entry, DashMap};
use fxhash::FxHasher;
use game_data_info_struct_reader::ModelDataPoint;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

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
