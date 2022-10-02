use actix_broadcaster::{ActixBroadcaster, Broadcaster};
use dashmap::DashMap;
use fxhash::FxHasher;
use game_data_info_struct_reader::ModelDataPoint;
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use std::hash::BuildHasherDefault;
use std::sync::Arc;

/// Process a datapoint into all available models, and broadcast model emmissions, retaining only (model, broadcaster) pairs for which the model needs no update or the broadcaster has clients
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
