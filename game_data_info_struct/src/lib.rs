pub mod date;

use chrono::NaiveDate;
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
    pub nanites: f64,
}

/// Represent inflow/outflow streams for the 17 base resources. Maps a resource to a `M` where `M` maps a producer/consumer to an amount
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BudgetComponent {
    pub energy: HashMap<String, f64>,
    pub minerals: HashMap<String, f64>,
    pub food: HashMap<String, f64>,

    pub physics_research: HashMap<String, f64>,
    pub society_research: HashMap<String, f64>,
    pub engineering_research: HashMap<String, f64>,

    pub influence: HashMap<String, f64>,
    pub unity: HashMap<String, f64>,
    pub consumer_goods: HashMap<String, f64>,

    pub alloys: HashMap<String, f64>,

    pub volatile_motes: HashMap<String, f64>,
    pub exotic_gases: HashMap<String, f64>,
    pub rare_crystals: HashMap<String, f64>,

    pub sr_living_metal: HashMap<String, f64>,
    pub sr_zro: HashMap<String, f64>,
    pub sr_dark_matter: HashMap<String, f64>,
    pub nanites: HashMap<String, f64>,
}

pub trait Index<T> {
    fn index<'a, 'b>(&'a self, res: &'b ResourceClass) -> &'a T;
}
pub trait IndexMut<T> {
    fn index_mut<'a, 'b>(&'a mut self, res: &'b ResourceClass) -> &'a mut T;
}

impl IndexMut<f64> for Resources {
    fn index_mut<'a, 'b>(&'a mut self, res: &'b ResourceClass) -> &'a mut f64 {
        match res {
            ResourceClass::Energy => &mut self.energy,
            ResourceClass::Minerals => &mut self.minerals,
            ResourceClass::Food => &mut self.food,
            ResourceClass::Physics => &mut self.physics_research,
            ResourceClass::Society => &mut self.society_research,
            ResourceClass::Engineering => &mut self.engineering_research,
            ResourceClass::Influence => &mut self.influence,
            ResourceClass::Unity => &mut self.unity,
            ResourceClass::ConsumerGoods => &mut self.consumer_goods,
            ResourceClass::Alloys => &mut self.alloys,
            ResourceClass::Motes => &mut self.volatile_motes,
            ResourceClass::Gasses => &mut self.exotic_gases,
            ResourceClass::Crystals => &mut self.rare_crystals,
            ResourceClass::LivingMetal => &mut self.sr_living_metal,
            ResourceClass::Zro => &mut self.sr_zro,
            ResourceClass::DarkMatter => &mut self.sr_dark_matter,
            ResourceClass::Nanites => &mut self.nanites,
        }
    }
}

impl IndexMut<HashMap<String, f64>> for BudgetComponent {
    fn index_mut<'a, 'b>(&'a mut self, res: &'b ResourceClass) -> &'a mut HashMap<String, f64> {
        match res {
            ResourceClass::Energy => &mut self.energy,
            ResourceClass::Minerals => &mut self.minerals,
            ResourceClass::Food => &mut self.food,
            ResourceClass::Physics => &mut self.physics_research,
            ResourceClass::Society => &mut self.society_research,
            ResourceClass::Engineering => &mut self.engineering_research,
            ResourceClass::Influence => &mut self.influence,
            ResourceClass::Unity => &mut self.unity,
            ResourceClass::ConsumerGoods => &mut self.consumer_goods,
            ResourceClass::Alloys => &mut self.alloys,
            ResourceClass::Motes => &mut self.volatile_motes,
            ResourceClass::Gasses => &mut self.exotic_gases,
            ResourceClass::Crystals => &mut self.rare_crystals,
            ResourceClass::LivingMetal => &mut self.sr_living_metal,
            ResourceClass::Zro => &mut self.sr_zro,
            ResourceClass::DarkMatter => &mut self.sr_dark_matter,
            ResourceClass::Nanites => &mut self.nanites,
        }
    }
}

impl Index<f64> for Resources {
    fn index<'a, 'b>(&'a self, res: &'b ResourceClass) -> &'a f64 {
        match res {
            ResourceClass::Energy => &self.energy,
            ResourceClass::Minerals => &self.minerals,
            ResourceClass::Food => &self.food,
            ResourceClass::Physics => &self.physics_research,
            ResourceClass::Society => &self.society_research,
            ResourceClass::Engineering => &self.engineering_research,
            ResourceClass::Influence => &self.influence,
            ResourceClass::Unity => &self.unity,
            ResourceClass::ConsumerGoods => &self.consumer_goods,
            ResourceClass::Alloys => &self.alloys,
            ResourceClass::Motes => &self.volatile_motes,
            ResourceClass::Gasses => &self.exotic_gases,
            ResourceClass::Crystals => &self.rare_crystals,
            ResourceClass::LivingMetal => &self.sr_living_metal,
            ResourceClass::Zro => &self.sr_zro,
            ResourceClass::DarkMatter => &self.sr_dark_matter,
            ResourceClass::Nanites => &self.nanites,
        }
    }
}

impl Index<HashMap<String, f64>> for BudgetComponent {
    fn index<'a, 'b>(&'a self, res: &'b ResourceClass) -> &'a HashMap<String, f64> {
        match res {
            ResourceClass::Energy => &self.energy,
            ResourceClass::Minerals => &self.minerals,
            ResourceClass::Food => &self.food,
            ResourceClass::Physics => &self.physics_research,
            ResourceClass::Society => &self.society_research,
            ResourceClass::Engineering => &self.engineering_research,
            ResourceClass::Influence => &self.influence,
            ResourceClass::Unity => &self.unity,
            ResourceClass::ConsumerGoods => &self.consumer_goods,
            ResourceClass::Alloys => &self.alloys,
            ResourceClass::Motes => &self.volatile_motes,
            ResourceClass::Gasses => &self.exotic_gases,
            ResourceClass::Crystals => &self.rare_crystals,
            ResourceClass::LivingMetal => &self.sr_living_metal,
            ResourceClass::Zro => &self.sr_zro,
            ResourceClass::DarkMatter => &self.sr_dark_matter,
            ResourceClass::Nanites => &self.nanites,
        }
    }
}
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub income: BudgetComponent,
    pub expense: BudgetComponent,
}

pub const ALL_RESOURCES: [ResourceClass; 17] = [
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
    ResourceClass::Nanites,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
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
    Nanites,
}

pub trait ResourceCode {
    fn code(&self) -> &str;
}

impl ResourceCode for ResourceClass {
    fn code(&self) -> &str {
        match self {
            ResourceClass::Energy => "Energy",
            ResourceClass::Minerals => "Minerals",
            ResourceClass::Food => "Food",
            ResourceClass::Physics => "Physics",
            ResourceClass::Society => "Society",
            ResourceClass::Engineering => "Engineering",
            ResourceClass::Influence => "Influence",
            ResourceClass::Unity => "Unity",
            ResourceClass::ConsumerGoods => "ConsumerGoods",
            ResourceClass::Alloys => "Alloys",
            ResourceClass::Motes => "Motes",
            ResourceClass::Gasses => "Gasses",
            ResourceClass::Crystals => "Crystals",
            ResourceClass::LivingMetal => "LivingMetal",
            ResourceClass::Zro => "Zro",
            ResourceClass::DarkMatter => "DarkMatter",
            ResourceClass::Nanites => "Nanites",
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
                ResourceClass::Nanites => "nanites",
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
