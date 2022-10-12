mod extractor;
mod util;
use extractor::{
    budget::BudgetExtractor, empires::EmpiresExtractor, fleet::FleetExtractor,
    fleets::FleetsExtractor, resources::ResourcesExtractor, Extractor,
};

use clausewitz_parser::{ClausewitzValue, Val};
use directory_watcher::FileReader;
pub use game_data_info_struct::{
    Budget, EmpireData, ModelDataPoint, PlayerClass, ResourceClass, Resources,
};
use std::path::Path;
pub struct GameDataInfoStructReader;

impl FileReader for GameDataInfoStructReader {
    type OUT = ModelDataPoint;
    fn read_file(&self, file: &Path) -> ModelDataPoint {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::cheat_root(
            &gamestate_raw,
            vec![
                "version",
                "player",
                "country",
                "fleet",
                "ships",
                "ship_design",
            ],
        )
        .unwrap();

        Self::extract(meta_val, gamestate_val)
    }
}

impl GameDataInfoStructReader {
    fn extract(meta: Val, gamestate: Val) -> ModelDataPoint {
        let country = gamestate.get_array_at_path("country").expect("array `country` not found in parsed gamestate. Something has gone wrong, check your parser!");
        let fleets = gamestate.get_array_at_path("fleet").expect("array `country` not found in parsed gamestate. Something has gone wrong, check your parser!");
        let ships = gamestate.get_array_at_path("ships").expect("array `country` not found in parsed gamestate. Something has gone wrong, check your parser!");
        let ship_design = gamestate.get_array_at_path("ship_design").expect("array `country` not found in parsed gamestate. Something has gone wrong, check your parser!");
        let empires = if let Ok(v) = gamestate.get_set_at_path("player") {
            EmpiresExtractor::create(country, v, fleets, ships, ship_design).extract()
        } else {
            EmpiresExtractor::create(country, &vec![], fleets, ships, ship_design).extract()
        };

        ModelDataPoint {
            campaign_name: meta.get_string_at_path("name").expect("key `name` not found in parsed meta file. Something has gone wrong, check your parser!").to_string(),
            date: meta.get_date_at_path("date").expect("key `date` not found in parsed meta file. Something has gone wrong, check your parser!").to_owned().into(),
            empires: empires,
        }
    }
}
