use super::{fleet::MilitaryFleetExtractor, Extractor};
use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::fleet::Fleets;

pub(crate) struct FleetsExtractor<'a> {
    owned_fleets: &'a Vec<Val<'a>>,

    fleets: &'a Vec<(u64, Val<'a>)>,
    ships: &'a Vec<(u64, Val<'a>)>,
    ship_design: &'a Vec<(u64, Val<'a>)>,
}

impl<'a> Extractor for FleetsExtractor<'a> {
    type Yield = Fleets;

    fn extract(&self) -> Fleets {
        let fleets = self
            .owned_fleets
            .iter()
            .filter_map(|fleet_dict| {
                let fleet_id = *fleet_dict.get_integer_at_path("fleet").unwrap() as u64;
                let fleet = &self
                    .fleets
                    .iter()
                    .find_map(|(i, fleet)| if i == &fleet_id { Some(fleet) } else { None })
                    .unwrap();
                MilitaryFleetExtractor::create(fleet, self.ships, self.ship_design).extract()
            })
            .collect::<Vec<_>>();
        Fleets { military: fleets }
    }
}
impl<'a> FleetsExtractor<'a> {
    pub fn create(
        owned_fleets: &'a Vec<Val<'a>>,

        fleets: &'a Vec<(u64, Val<'a>)>,
        ships: &'a Vec<(u64, Val<'a>)>,
        ship_design: &'a Vec<(u64, Val<'a>)>,
    ) -> FleetsExtractor<'a> {
        FleetsExtractor {
            owned_fleets,
            fleets,
            ships,
            ship_design,
        }
    }
}
