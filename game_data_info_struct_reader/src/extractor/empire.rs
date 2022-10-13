use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::{empire::EmpireData, player::PlayerClass};

use crate::{util, Extractor};

use super::{budget::BudgetExtractor, fleets::FleetsExtractor, resources::ResourcesExtractor};

pub(crate) struct EmpireExtractor<'a> {
    country: &'a Val<'a>,
    player_class: PlayerClass,
    fleets: &'a Vec<(u64, Val<'a>)>,
    ships: &'a Vec<(u64, Val<'a>)>,
    ship_design: &'a Vec<(u64, Val<'a>)>,
}

impl<'a> Extractor for EmpireExtractor<'a> {
    type Yield = Option<EmpireData>;

    fn extract(&self) -> Option<EmpireData> {
        if let Err(_) = self.country.get_at_path("victory_rank") {
            return None;
        } else if let Err(_) = self.country.get_at_path("owned_planets") {
            return None;
        } else if let Ok(standard_economy_module) =
            self.country.get_at_path("modules.standard_economy_module")
        {
            let owned_fleets = self
                .country
                .get_set_at_path("fleets_manager.owned_fleets")
                .unwrap();

            Some(EmpireData {
                name: Self::extract_empire_name(self.country),
                driver: self.player_class.clone(),
                budget: BudgetExtractor::create(self.country.get_at_path("budget").unwrap())
                    .extract(),
                resources: ResourcesExtractor::create(
                    standard_economy_module.get_at_path("resources").unwrap(),
                )
                .extract(),
                fleets: FleetsExtractor::create(
                    owned_fleets,
                    self.fleets,
                    self.ships,
                    self.ship_design,
                )
                .extract(),
            })
        } else {
            None
        }
    }
}
impl<'a> EmpireExtractor<'a> {
    pub fn create(
        country: &'a Val<'a>,
        player_class: PlayerClass,

        fleets: &'a Vec<(u64, Val<'a>)>,
        ships: &'a Vec<(u64, Val<'a>)>,
        ship_design: &'a Vec<(u64, Val<'a>)>,
    ) -> EmpireExtractor<'a> {
        EmpireExtractor {
            country,
            player_class,
            fleets,
            ships,
            ship_design,
        }
    }

    fn extract_empire_name(country: &Val) -> String {
        let mut parts = vec![];
        let mut spec: Option<String> = None;

        if let Ok(variables) = country.get_set_at_path("name.variables") {
            for v in variables {
                if let (Ok(key), Ok(value_key)) = (
                    v.get_string_at_path("key"),
                    v.get_string_at_path("value.key"),
                ) {
                    if !key.contains("This.") {
                        parts.push(util::strip_name(value_key))
                    } else {
                        spec = Some(util::strip_name(value_key))
                    }
                }
            }
        } else if let Ok(key) = country.get_string_at_path("name.key") {
            parts.push(key.replace("NAME", ""))
        }
        if let Some(spec) = spec {
            parts.insert(0, spec);
        }
        let ret = parts.join(" ");

        ret
    }
}
