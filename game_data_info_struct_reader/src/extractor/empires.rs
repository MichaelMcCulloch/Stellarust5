use clausewitz_parser::Val;
use game_data_info_struct::EmpireData;

use crate::Extractor;

pub(crate) struct EmpiresExtractor {}

impl Extractor for EmpiresExtractor {
    type Yield = Vec<EmpireData>;

    fn extract(val: &Val) -> Vec<EmpireData> {
        todo!()
    }
}
