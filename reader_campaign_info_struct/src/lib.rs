use std::path::Path;

use campaign_info_struct::CampaignInfoStruct;
use clausewitz_value::Val;
use directory_watcher::FileReader;

pub struct CampaignInfoStructReader;

impl FileReader for CampaignInfoStructReader {
    type OUT = CampaignInfoStruct;
    fn read_file(&self, file: &Path) -> CampaignInfoStruct {
        let (meta_raw, gamestate_raw) = game_data_unzipper::get_zipped_content(file);
        let (_, meta_val) = clausewitz_parser::root(&meta_raw).unwrap();
        let (_, gamestate_val) = clausewitz_parser::root(&gamestate_raw).unwrap();
        self.extract(&meta_val, &gamestate_val)
    }
}

impl CampaignInfoStructReader {
    fn extract(&self, meta: &Val, gamestate: &Val) -> CampaignInfoStruct {
        CampaignInfoStruct {
            campaign_name: meta.get_string_at_path("name").unwrap().to_owned(),
        }
    }
}
