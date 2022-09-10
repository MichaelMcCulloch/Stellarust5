use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Empire {
    pub name: String,
    pub player: Option<String>,
}

// Clone is only used to make an ownable copy to serialize and send.
#[derive(Serialize, Deserialize, Debug)]
pub struct CampaignInfoStruct {
    pub campaign_name: String,
    pub empire_list: Vec<Empire>,
}
