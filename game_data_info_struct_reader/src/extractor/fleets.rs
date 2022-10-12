use clausewitz_parser::Val;
use game_data_info_struct::Fleets;

use crate::Extractor;

pub(crate) struct FleetsExtractor<'a> {
    owned_fleets: &'a Vec<Val<'a>>,
    fleets: &'a Vec<Val<'a>>,
    ships: &'a Vec<Val<'a>>,
    ship_design: &'a Vec<Val<'a>>,
}

impl<'a> Extractor for FleetsExtractor<'a> {
    type Yield = Fleets;

    fn extract(&self) -> Fleets {
        Fleets { military: vec![] }
    }
}
impl<'a> FleetsExtractor<'a> {
    pub fn create(
        owned_fleets: &'a Vec<Val<'a>>,
        fleets: &'a Vec<Val<'a>>,
        ships: &'a Vec<Val<'a>>,
        ship_design: &'a Vec<Val<'a>>,
    ) -> FleetsExtractor<'a> {
        FleetsExtractor {
            owned_fleets,
            fleets,
            ships,
            ship_design,
        }
    }
}
