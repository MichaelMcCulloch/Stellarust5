mod extractor;
use extractor::{
    budget::BudgetExtractor, fleet::FleetExtractor, resources::ResourcesExtractor, Extractor,
};

use clausewitz_parser::{ClausewitzValue, Val};
use directory_watcher::FileReader;
pub use game_data_info_struct::{
    Budget, EmpireData, ModelDataPoint, PlayerClass, ResourceClass, Resources,
};
use game_data_info_struct::{BudgetComponent, IndexMut, ALL_RESOURCES};
use std::path::Path;
pub struct GameDataInfoStructReader;

impl FileReader for GameDataInfoStructReader {
    type OUT = ModelDataPoint;
    fn read_file(&self, file: &Path) -> ModelDataPoint {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::cheat_root(&gamestate_raw).unwrap();

        Self::extract(meta_val, gamestate_val)
    }
}

impl GameDataInfoStructReader {
    fn extract_empire(country: &Val, player_class: PlayerClass) -> Option<EmpireData> {
        if let Err(_) = country.get_at_path("victory_rank") {
            return None;
        } else if let Err(_) = country.get_at_path("owned_planets") {
            return None;
        } else if let Ok(standard_economy_module) =
            country.get_at_path("modules.standard_economy_module")
        {
            Some(EmpireData {
                name: Self::extract_empire_name(country),
                driver: player_class,
                budget: BudgetExtractor::create(country.get_at_path("budget").unwrap()).extract(),
                resources: ResourcesExtractor::create(
                    standard_economy_module.get_at_path("resources").unwrap(),
                )
                .extract(),
            })
        } else {
            None
        }
    }
    fn extract_empires(countries: &Vec<Val>, players: &Vec<Val>) -> Vec<EmpireData> {
        let mut handled = vec![];
        for player in players.iter() {
            let player_name = player.get_string_at_path("name").unwrap();
            let player_country_index = player.get_integer_at_path("country").unwrap();

            handled.push((
                player_country_index,
                PlayerClass::Human(player_name.to_string()),
            ));
        }
        handled.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut empires = vec![];
        for (idx, country) in countries.iter().enumerate() {
            let player_class = if let Some(i) = handled.get(0).map(|(i, _)| i) {
                if i == &&(idx as i64) {
                    handled.remove(0).1
                } else {
                    PlayerClass::Computer
                }
            } else {
                PlayerClass::Computer
            };
            if let Some(country) = Self::extract_empire(country, player_class) {
                empires.push(country)
            }
        }
        empires
    }

    fn extract(meta: Val, gamestate: Val) -> ModelDataPoint {
        let country = gamestate.get_array_at_path("country").expect("array `country` not found in parsed gamestate. Something has gone wrong, check your parser!");
        let empires = if let Ok(v) = gamestate.get_set_at_path("player") {
            Self::extract_empires(country, v)
        } else {
            Self::extract_empires(country, &vec![])
        };

        ModelDataPoint {
            campaign_name: meta.get_string_at_path("name").expect("key `name` not found in parsed meta file. Something has gone wrong, check your parser!").to_string(),
            date: meta.get_date_at_path("date").expect("key `date` not found in parsed meta file. Something has gone wrong, check your parser!").to_owned().into(),
            empires: empires,
        }
    }

    fn strip_name(name: &str) -> String {
        let mut ret = name.to_string();
        let rm = vec!["SPEC_", "SPEC", "_system", "_planet", "NAME"];
        for r in rm {
            ret = ret.replace(r, "");
        }
        ret = ret.replace("_", " ");
        ret
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
                        parts.push(Self::strip_name(value_key))
                    } else {
                        spec = Some(Self::strip_name(value_key))
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
