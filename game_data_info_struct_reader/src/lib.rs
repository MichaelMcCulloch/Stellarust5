use clausewitz_parser::{ClausewitzValue, Val};
use directory_watcher::FileReader;
pub use game_data_info_struct::{
    Budget, EmpireData, ModelDataPoint, PlayerClass, ResourceClass, Resources,
};
use std::{collections::HashMap, path::Path};
pub struct GameDataInfoStructReader;

impl FileReader for GameDataInfoStructReader {
    type OUT = ModelDataPoint;
    fn read_file(&self, file: &Path) -> ModelDataPoint {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::root(&gamestate_raw).unwrap();

        Self::extract(&meta_val, &gamestate_val)
    }
}

pub const ALL_RESOURCES: [(ResourceClass, &str); 16] = [
    (ResourceClass::Energy, "energy"),
    (ResourceClass::Minerals, "minerals"),
    (ResourceClass::Food, "food"),
    (ResourceClass::Physics, "physics_research"),
    (ResourceClass::Society, "society_research"),
    (ResourceClass::Engineering, "engineering_research"),
    (ResourceClass::Influence, "influence"),
    (ResourceClass::Unity, "unity"),
    (ResourceClass::ConsumerGoods, "consumer_goods"),
    (ResourceClass::Alloys, "alloys"),
    (ResourceClass::Motes, "volatile_motes"),
    (ResourceClass::Gasses, "exotic_gases"),
    (ResourceClass::Crystals, "rare_crystals"),
    (ResourceClass::LivingMetal, "sr_living_metal"),
    (ResourceClass::Zro, "sr_zro"),
    (ResourceClass::DarkMatter, "sr_dark_matter"),
];

impl GameDataInfoStructReader {
    fn extract_budget(budget: &Val) -> Budget {
        let current_month_budget = budget.get_at_path("current_month").unwrap();
        let last_month_budget = budget.get_at_path("last_month").unwrap();

        let get_budget_val =
            |key: &str, budget_period: &Val| -> HashMap<ResourceClass, Vec<(String, f64)>> {
                Self::get_budget_component_map(budget_period.get_at_path(key).unwrap())
            };

        Budget {
            income: get_budget_val("income", current_month_budget),
            expense: get_budget_val("expenses", current_month_budget),
            balance: get_budget_val("balance", current_month_budget),
            income_last_month: get_budget_val("income", last_month_budget),
            expense_last_month: get_budget_val("income", last_month_budget),
            balance_last_month: get_budget_val("income", last_month_budget),
        }
    }
    fn get_budget_component_map(component: &Val<'_>) -> HashMap<ResourceClass, Vec<(String, f64)>> {
        if let Val::Dict(sources) = component {
            let map = sources.into_iter().fold(
                HashMap::new(),
                |mut map, (contributor, contributions)| {
                    let contribitions_per_class = Self::get_contributions_per_class(contributions);

                    for (key, amount) in contribitions_per_class.into_iter() {
                        map.entry(key)
                            .or_insert(vec![])
                            .push((String::from(*contributor), amount));
                    }
                    map
                },
            );
            map
        } else {
            panic!()
        }
    }
    fn get_contributions_per_class(contributions: &Val<'_>) -> Vec<(ResourceClass, f64)> {
        ALL_RESOURCES
            .iter()
            .filter_map(|(class, key)| {
                if let Ok(val) = contributions.get_at_path(key) {
                    match val {
                        Val::Decimal(d) => Some((class.clone(), *d)),
                        Val::Integer(i) => Some((class.clone(), *i as f64)),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }
    fn extract_resources(resources: &Val) -> Resources {
        let extract_resource = |resource_path: &str| -> f64 {
            if let Ok(x) = resources.get_number_at_path(resource_path) {
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
        }
    }
    fn extract_empire(country: &Val, player_class: PlayerClass) -> Option<EmpireData> {
        match (
            country.get_at_path("modules.standard_economy_module"),
            player_class,
        ) {
            (Err(_), PlayerClass::Human(_)) => {
                log::error!("dict `modules.standard_economy_module` not found in a country assigned to a human. Something has gone wrong, check your parser!");
                None
            }
            (Err(_), PlayerClass::Computer) => None,
            (Ok(standard_economy_module), class) => Some(EmpireData {
                name: country.get_string_at_path("name.key").unwrap().to_string(),
                driver: class,
                budget: Self::extract_budget(country.get_at_path("budget").unwrap()),
                resources: Self::extract_resources(
                    standard_economy_module.get_at_path("resources").unwrap(),
                ),
            }),
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

    fn extract(meta: &Val, gamestate: &Val) -> ModelDataPoint {
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
}

#[cfg(test)]
mod test;
#[cfg(test)]
mod tests {

    use crate::test::constant_str::{
        COMPLETE_MODEL_SERIALIZED, EMPIRE_DATA_SERIALIZED, GAMESTATE, META,
    };

    use super::*;
    #[test]
    fn verify_model_consistent() {
        let actual = GameDataInfoStructReader::extract(
            &clausewitz_parser::root(&META).unwrap().1,
            &clausewitz_parser::root(&GAMESTATE).unwrap().1,
        );

        let expected: ModelDataPoint = serde_json::from_str(&COMPLETE_MODEL_SERIALIZED).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_empires____() {
        let gamestate = &clausewitz_parser::root(&GAMESTATE).unwrap().1;
        let countries = gamestate.get_array_at_path("country").unwrap();
        let players = gamestate.get_set_at_path("player").unwrap();

        let actual = GameDataInfoStructReader::extract_empires(countries, players);

        let expected: Vec<EmpireData> = serde_json::from_str(&EMPIRE_DATA_SERIALIZED).unwrap();
        assert_eq!(expected, actual);
    }
}
