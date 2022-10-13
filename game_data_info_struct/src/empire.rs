use serde_derive::{Deserialize, Serialize};

use crate::{budget::Budget, fleet::Fleets, player::PlayerClass, resource::Resources};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmpireData {
    pub name: String,
    pub driver: PlayerClass,
    pub budget: Budget,
    pub resources: Resources,
    pub fleets: Fleets,
}
