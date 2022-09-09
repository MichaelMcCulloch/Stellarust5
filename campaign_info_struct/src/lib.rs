use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignInfoStruct {
    pub campaign_name: String,
}
