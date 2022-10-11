use clausewitz_parser::Val;

pub(crate) mod budget;
pub(crate) mod fleet;

pub(crate) trait Extractor {
    type Yield;

    fn extract(val: &Val) -> Self::Yield;
}
