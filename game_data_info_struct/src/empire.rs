use crate::{budget::Budget, fleet::Fleets, player::PlayerClass, resource::Resources};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmpireData {
    pub name: String,
    pub driver: PlayerClass,
    pub budget: Budget,
    pub resources: Resources,
    pub fleets: Fleets,
}
