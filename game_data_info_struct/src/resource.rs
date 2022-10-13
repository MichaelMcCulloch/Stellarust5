use crate::{index::Index, index_mut::IndexMut};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
