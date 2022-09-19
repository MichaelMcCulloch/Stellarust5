use actix_broadcaster::Client;
use model_info_struct::enums::ModelSpecEnum;
pub mod controller;
pub mod model_controller;
pub(crate) mod spawn_event_loop;
pub trait Controller {
    fn get_client(&self, model_spec_enum: ModelSpecEnum) -> Option<Client>;
}
