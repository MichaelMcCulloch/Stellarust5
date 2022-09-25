pub mod date;

use chrono::NaiveDate;
use date::Date;
use std::{collections::HashMap, fmt::Display};

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
    pub income: [Vec<(String, f64)>; 16],
    pub expense: [Vec<(String, f64)>; 16],
    pub balance: [Vec<(String, f64)>; 16],

    pub income_last_month: [Vec<(String, f64)>; 16],
    pub expense_last_month: [Vec<(String, f64)>; 16],
    pub balance_last_month: [Vec<(String, f64)>; 16],
}

pub const ALL_RESOURCES: [ResourceClass; 16] = [
    ResourceClass::Energy,
    ResourceClass::Minerals,
    ResourceClass::Food,
    ResourceClass::Physics,
    ResourceClass::Society,
    ResourceClass::Engineering,
    ResourceClass::Influence,
    ResourceClass::Unity,
    ResourceClass::ConsumerGoods,
    ResourceClass::Alloys,
    ResourceClass::Motes,
    ResourceClass::Gasses,
    ResourceClass::Crystals,
    ResourceClass::LivingMetal,
    ResourceClass::Zro,
    ResourceClass::DarkMatter,
];

pub trait BudgetMapIndex {
    fn index(&self) -> usize;
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

impl BudgetMapIndex for ResourceClass {
    fn index(&self) -> usize {
        match self {
            ResourceClass::Energy => 0,
            ResourceClass::Minerals => 1,
            ResourceClass::Food => 2,
            ResourceClass::Physics => 3,
            ResourceClass::Society => 4,
            ResourceClass::Engineering => 5,
            ResourceClass::Influence => 6,
            ResourceClass::Unity => 7,
            ResourceClass::ConsumerGoods => 8,
            ResourceClass::Alloys => 9,
            ResourceClass::Motes => 10,
            ResourceClass::Gasses => 11,
            ResourceClass::Crystals => 12,
            ResourceClass::LivingMetal => 13,
            ResourceClass::Zro => 14,
            ResourceClass::DarkMatter => 15,
        }
    }
}

impl Display for ResourceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResourceClass::Energy => "energy",
                ResourceClass::Minerals => "minerals",
                ResourceClass::Food => "food",
                ResourceClass::Physics => "physics_research",
                ResourceClass::Society => "society_research",
                ResourceClass::Engineering => "engineering_research",
                ResourceClass::Influence => "influence",
                ResourceClass::Unity => "unity",
                ResourceClass::ConsumerGoods => "consumer_goods",
                ResourceClass::Alloys => "alloys",
                ResourceClass::Motes => "volatile_motes",
                ResourceClass::Gasses => "exotic_gases",
                ResourceClass::Crystals => "rare_crystals",
                ResourceClass::LivingMetal => "sr_living_metal",
                ResourceClass::Zro => "sr_zro",
                ResourceClass::DarkMatter => "sr_dark_matter",
            }
        )
    }
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
    #[serde(with = "naive_date_serde")]
    pub date: NaiveDate,
    pub empires: Vec<EmpireData>,
}

mod naive_date_serde {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::date::Date;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let x = Date::from(date);
        x.serialize(serializer)
    }

    pub fn deserialize<'de, D>(input: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        Date::deserialize(input)
            .map(|date| NaiveDate::from_ymd(date.0.into(), date.1.into(), date.2.into()))
    }
}
