use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CampaignInfoStruct {
    pub campaign_name: String,
}
