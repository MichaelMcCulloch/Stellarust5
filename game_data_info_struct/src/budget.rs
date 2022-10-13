use crate::{index::Index, index_mut::IndexMut, resource::ResourceClass};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub income: BudgetComponent,
    pub expense: BudgetComponent,
}
