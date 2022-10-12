use clausewitz_parser::Val;

pub(crate) mod budget;
pub(crate) mod empire;
pub(crate) mod empires;
pub(crate) mod fleet;
pub(crate) mod fleets;
pub(crate) mod resources;

pub(crate) trait Extractor {
    type Yield;

    fn extract(&self) -> Self::Yield;
}
