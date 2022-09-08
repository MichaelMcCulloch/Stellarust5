use campaign_info_struct::CampaignInfoStruct;
use directory_watcher::FileReader;

pub struct CampaignInfoStructReader;

impl FileReader<CampaignInfoStruct> for CampaignInfoStructReader {
    fn read_file(&self, file: &std::path::Path) -> CampaignInfoStruct {
        todo!()
    }
}
