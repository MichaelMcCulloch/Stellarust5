use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fleets {
    pub military: Vec<Fleet>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub class: ShipClass,

    pub hitpoints: f64,
    pub shield_hitpoints: f64,
    pub armor_hitpoints: f64,

    pub max_hitpoints: f64,
    pub max_shield_hitpoints: f64,
    pub max_armor_hitpoints: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub ships: Vec<Ship>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ShipClass {
    Corvette,
    Destroyer,
    Cruiser,
    Battleship,
    Titan,
    Juggernaut,
    Colossus,
}
