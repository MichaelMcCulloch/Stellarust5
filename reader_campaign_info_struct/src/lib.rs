use std::path::Path;
mod player_kinds;
use campaign_info_struct::{CampaignInfoStruct, Empire};
use clausewitz_parser::{ClausewitzValue, IndexError, Val};
use directory_watcher::FileReader;
use player_kinds::PlayerKind;
pub struct CampaignInfoStructReader;

impl FileReader for CampaignInfoStructReader {
    type OUT = CampaignInfoStruct;
    fn read_file(&self, file: &Path) -> CampaignInfoStruct {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::root(&gamestate_raw).unwrap();
        self.extract(&meta_val, &gamestate_val)
    }
}

impl CampaignInfoStructReader {
    fn extract(&self, meta: &Val<'_>, gamestate: &Val) -> CampaignInfoStruct {
        CampaignInfoStruct {
            campaign_name: meta.get_string_at_path("name").unwrap().to_owned(),
            empire_list: Self::get_countries(gamestate),
        }
    }
    fn get_countries(gamestate: &Val) -> Vec<Empire> {
        let countries = match gamestate.get_array_at_path("country") {
            Ok(countries_vec) => countries_vec,
            Err(_) => panic!("Cannot proceed without country array in gamestate!!!"),
        };
        let player_list = gamestate
            .get_set_at_path("player")
            .unwrap()
            .iter()
            .map(|val| {
                (
                    val.get_string_at_path("name").unwrap(),
                    val.get_integer_at_path("country").unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let country_list = countries
            .iter()
            .enumerate()
            .filter_map(|(index, country)| {
                let player_kind = if let Some(name) =
                    player_list.iter().find_map(|(name, country)| {
                        if &&(index as i64) == country {
                            Some(name)
                        } else {
                            None
                        }
                    }) {
                    PlayerKind::Human(name.to_string())
                } else {
                    PlayerKind::Machine
                };
                match Self::create_empire_data(country, player_kind) {
                    Ok(empire_data) => Some(empire_data),
                    Err(_) => None,
                }
            })
            .collect();
        country_list
    }
    fn create_empire_data(country: &Val, player_kind: PlayerKind) -> Result<Empire, IndexError> {
        Ok(Empire {
            name: String::from(country.get_string_at_path("name")?),
            player: match player_kind {
                PlayerKind::Human(name) => Some(name),
                PlayerKind::Machine => None,
            },
        })
    }
}
