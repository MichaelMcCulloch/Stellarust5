use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::{EmpireData, PlayerClass};

use crate::Extractor;

use super::empire::EmpireExtractor;

pub(crate) struct EmpiresExtractor<'a> {
    countries: &'a Vec<(u64, Val<'a>)>,
    players: &'a Vec<Val<'a>>,
    fleets: &'a Vec<(u64, Val<'a>)>,
    ships: &'a Vec<(u64, Val<'a>)>,
    ship_design: &'a Vec<(u64, Val<'a>)>,
}

impl<'a> Extractor for EmpiresExtractor<'a> {
    type Yield = Vec<EmpireData>;

    fn extract(&self) -> Vec<EmpireData> {
        let mut handled = vec![];
        for player in self.players.iter() {
            let player_name = player.get_string_at_path("name").unwrap();
            let player_country_index = *player.get_integer_at_path("country").unwrap() as u64;

            handled.push((
                player_country_index,
                PlayerClass::Human(player_name.to_string()),
            ));
        }

        let mut empires = vec![];
        for (idx, country) in self.countries.iter() {
            let player_class = if let Some(i) = handled.get(0).map(|(i, _)| i) {
                if i == idx {
                    handled.remove(0).1
                } else {
                    PlayerClass::Computer
                }
            } else {
                PlayerClass::Computer
            };
            if let Some(country) = EmpireExtractor::create(
                country,
                player_class,
                self.fleets,
                self.ships,
                self.ship_design,
            )
            .extract()
            {
                empires.push(country)
            }
        }
        empires
    }
}

impl<'a> EmpiresExtractor<'a> {
    pub fn create(
        countries: &'a Vec<(u64, Val<'a>)>,
        players: &'a Vec<Val<'a>>,
        fleets: &'a Vec<(u64, Val<'a>)>,
        ships: &'a Vec<(u64, Val<'a>)>,
        ship_design: &'a Vec<(u64, Val<'a>)>,
    ) -> EmpiresExtractor<'a> {
        EmpiresExtractor {
            countries,
            players,
            fleets,
            ships,
            ship_design,
        }
    }
}
