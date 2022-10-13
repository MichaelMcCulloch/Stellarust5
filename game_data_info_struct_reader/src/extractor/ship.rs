use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::fleet::{Ship, ShipClass};

use crate::Extractor;

pub(crate) struct MilitaryShipExtractor<'a> {
    ship: &'a Val<'a>,
    ship_design: &'a Vec<(u64, Val<'a>)>,
}

impl<'a> Extractor for MilitaryShipExtractor<'a> {
    type Yield = Option<Ship>;
    fn extract(&self) -> Option<Ship> {
        let design_id = *self.ship.get_integer_at_path("ship_design").unwrap() as u64;
        let ship_design = self
            .ship_design
            .iter()
            .find_map(|(i, v)| if i == &design_id { Some(v) } else { None })
            .unwrap();
        let ship_class = ship_design.get_string_at_path("ship_size").unwrap();

        let ship_size = if ship_class.contains("corvette") {
            Some(ShipClass::Corvette)
        } else if ship_class.contains("destroyer") {
            Some(ShipClass::Destroyer)
        } else if ship_class.contains("cruiser") {
            Some(ShipClass::Cruiser)
        } else if ship_class.contains("battleship") {
            Some(ShipClass::Battleship)
        } else if ship_class.contains("titan") {
            Some(ShipClass::Titan)
        } else if ship_class.contains("juggernaut") {
            Some(ShipClass::Juggernaut)
        } else if ship_class.contains("colossus") {
            Some(ShipClass::Corvette)
        } else {
            None
        };

        ship_size.map(|class| Ship {
            class,
            hitpoints: self
                .ship
                .get_number_at_path("hitpoints")
                .unwrap_or_default(),
            shield_hitpoints: self
                .ship
                .get_number_at_path("shield_hitpoints")
                .unwrap_or_default(),
            armor_hitpoints: self
                .ship
                .get_number_at_path("armor_hitpoints")
                .unwrap_or_default(),
            max_hitpoints: self
                .ship
                .get_number_at_path("max_hitpoints")
                .unwrap_or_default(),
            max_shield_hitpoints: self
                .ship
                .get_number_at_path("max_shield_hitpoints")
                .unwrap_or_default(),
            max_armor_hitpoints: self
                .ship
                .get_number_at_path("max_armor_hitpoints")
                .unwrap_or_default(),
        })
    }
}
impl<'a> MilitaryShipExtractor<'a> {
    pub fn create(
        ship: &'a Val<'a>,
        ship_design: &'a Vec<(u64, Val<'a>)>,
    ) -> MilitaryShipExtractor<'a> {
        MilitaryShipExtractor { ship, ship_design }
    }
}
