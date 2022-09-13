use clausewitz_parser::{ClausewitzValue, IndexError, Val};
use directory_watcher::FileReader;
use game_data_info_struct::ModelDataPoint;
use std::{path::Path, thread, time::Duration};
pub struct GameDataInfoStructReader;

impl FileReader for GameDataInfoStructReader {
    type OUT = ModelDataPoint;
    fn read_file(&self, file: &Path) -> ModelDataPoint {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::root(&gamestate_raw).unwrap();

        self.extract(&meta_val, &gamestate_val)
    }
}

impl GameDataInfoStructReader {
    fn extract(&self, meta: &Val, gamestate: &Val) -> ModelDataPoint {
        todo!()
    }
}
