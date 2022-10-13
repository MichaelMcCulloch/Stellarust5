use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::resource::{self, Resources};

use crate::Extractor;

pub(crate) struct ResourcesExtractor<'a> {
    resources: &'a Val<'a>,
}

impl<'a> Extractor for ResourcesExtractor<'a> {
    type Yield = resource::Resources;

    fn extract(&self) -> resource::Resources {
        let extract_resource = |resource_path: &str| -> f64 {
            if let Ok(x) = self.resources.get_number_at_path(resource_path) {
                x
            } else {
                0.0f64
            }
        };
        Resources {
            energy: extract_resource("energy"),
            minerals: extract_resource("minerals"),
            food: extract_resource("food"),
            physics_research: extract_resource("physics_research"),
            society_research: extract_resource("society_research"),
            engineering_research: extract_resource("engineering_research"),
            influence: extract_resource("influence"),
            unity: extract_resource("unity"),
            consumer_goods: extract_resource("consumer_goods"),
            alloys: extract_resource("alloys"),
            volatile_motes: extract_resource("volatile_motes"),
            exotic_gases: extract_resource("exotic_gases"),
            rare_crystals: extract_resource("rare_crystals"),
            sr_living_metal: extract_resource("sr_living_metal"),
            sr_zro: extract_resource("sr_zro"),
            sr_dark_matter: extract_resource("sr_dark_matter"),
            nanites: extract_resource("nanites"),
        }
    }
}

impl<'a> ResourcesExtractor<'a> {
    pub fn create(resources: &'a Val<'a>) -> ResourcesExtractor<'a> {
        ResourcesExtractor { resources }
    }
}
