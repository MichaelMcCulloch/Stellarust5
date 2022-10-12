use clausewitz_parser::Val;

use crate::Extractor;

pub(crate) struct FleetExtractor<'a> {
    country: &'a Val<'a>,
    fleet: &'a Val<'a>,
    ships: &'a Vec<Val<'a>>,
}

impl<'a> Extractor for FleetExtractor<'a> {
    type Yield = ();

    fn extract(&self) -> () {
        todo!()
    }
}
impl<'a> FleetExtractor<'a> {
    pub fn create(
        country: &'a Val<'a>,
        fleet: &'a Val<'a>,
        ships: &'a Vec<Val<'a>>,
    ) -> FleetExtractor<'a> {
        FleetExtractor {
            country,
            fleet,
            ships,
        }
    }
}
