pub mod date;

use date::Date;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub energy: f64,
    pub minerals: f64,
    pub food: f64,

    pub physics_research: f64,
    pub society_research: f64,
    pub engineering_research: f64,

    pub influence: f64,
    pub unity: f64,
    pub consumer_goods: f64,

    pub alloys: f64,

    pub volatile_motes: f64,
    pub exotic_gases: f64,
    pub rare_crystals: f64,

    pub sr_living_metal: f64,
    pub sr_zro: f64,
    pub sr_dark_matter: f64,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]

pub struct Budget {
    pub income: HashMap<ResourceClass, Vec<(String, f64)>>,
    pub expense: HashMap<ResourceClass, Vec<(String, f64)>>,
    pub balance: HashMap<ResourceClass, Vec<(String, f64)>>,

    pub income_last_month: HashMap<ResourceClass, Vec<(String, f64)>>,
    pub expense_last_month: HashMap<ResourceClass, Vec<(String, f64)>>,
    pub balance_last_month: HashMap<ResourceClass, Vec<(String, f64)>>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum ResourceClass {
    Energy,
    Minerals,
    Food,
    Physics,
    Society,
    Engineering,
    Influence,
    Unity,
    ConsumerGoods,
    Alloys,
    Motes,
    Gasses,
    Crystals,
    LivingMetal,
    Zro,
    DarkMatter,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

pub struct EmpireData {
    pub name: String,
    pub driver: PlayerClass,
    pub budget: Budget,
    pub resources: Resources,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

pub enum PlayerClass {
    Human(String),
    Computer,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

pub struct ModelDataPoint {
    pub campaign_name: String,
    pub date: Date,
    pub empires: Vec<EmpireData>,
}
