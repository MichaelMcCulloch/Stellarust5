use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::fleet::Fleet;

use crate::Extractor;

use super::ship::MilitaryShipExtractor;

pub(crate) struct MilitaryFleetExtractor<'a> {
    fleet: &'a Val<'a>,
    ships: &'a Vec<(u64, Val<'a>)>,
    ship_design: &'a Vec<(u64, Val<'a>)>,
}

impl<'a> Extractor for MilitaryFleetExtractor<'a> {
    type Yield = Option<Fleet>;

    fn extract(&self) -> Option<Fleet> {
        // let get_array_at_path = self.fleet.get_at_path("ships").unwrap();

        // log::warn!("{:?}", get_array_at_path);
        // exit(-1);
        let ships = self
            .fleet
            .get_set_at_path("ships")
            .unwrap()
            .into_iter()
            .filter_map(|ship_id| match ship_id {
                Val::Integer(index) => {
                    let ship = self
                        .ships
                        .iter()
                        .find_map(|(i, v)| if i == &(*index as u64) { Some(v) } else { None })
                        .unwrap();
                    MilitaryShipExtractor::create(ship, self.ship_design).extract()
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if ships.is_empty() {
            None
        } else {
            Some(Fleet { ships })
        }
    }
}
impl<'a> MilitaryFleetExtractor<'a> {
    pub fn create(
        fleet: &'a Val<'a>,
        ships: &'a Vec<(u64, Val<'a>)>,
        ship_design: &'a Vec<(u64, Val<'a>)>,
    ) -> MilitaryFleetExtractor<'a> {
        MilitaryFleetExtractor {
            fleet,
            ships,
            ship_design,
        }
    }
}
