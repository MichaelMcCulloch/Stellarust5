use crate::Extractor;

pub(crate) struct FleetExtractor {}

impl Extractor for FleetExtractor {
    type Yield = ();

    fn extract(&self) -> () {
        todo!()
    }
}
