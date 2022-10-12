use clausewitz_parser::Val;
use game_data_info_struct::Fleets;

use crate::Extractor;

pub(crate) struct FleetsExtractor<'a> {
    country: &'a Val<'a>,
    fleets: &'a Vec<Val<'a>>,
    ships: &'a Vec<Val<'a>>,
}

impl<'a> Extractor for FleetsExtractor<'a> {
    type Yield = Fleets;

    fn extract(&self) -> Fleets {
        todo!()
    }
}
impl<'a> FleetsExtractor<'a> {
    pub fn create(
        country: &'a Val<'a>,
        fleets: &'a Vec<Val<'a>>,
        ships: &'a Vec<Val<'a>>,
    ) -> FleetsExtractor<'a> {
        FleetsExtractor {
            country,
            fleets,
            ships,
        }
    }
}
