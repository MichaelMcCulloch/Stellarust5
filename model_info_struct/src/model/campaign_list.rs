use std::{collections::HashMap, vec};

use crate::{Model, ModelSpec};
use game_data_info_struct::ModelDataPoint;
use serde_derive::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Serialize, Clone, Debug)]
pub struct CampaignListModelSpec;

#[derive(Serialize, Debug)]
pub struct CampaignListModel {
    list: Vec<CampaignInfoStruct>,
}
impl Model for CampaignListModel {
    type ModelSpec = CampaignListModelSpec;

    fn create(_: Self::ModelSpec) -> Self {
        Self { list: vec![] }
    }

    type Representation = Vec<CampaignInfoStruct>;

    fn update(&mut self, game_data: &ModelDataPoint) -> Option<Self::Representation> {
        todo!()
    }

    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation> {
        todo!()
    }
}

impl ModelSpec for CampaignListModelSpec {
    type Model = CampaignListModel;
}

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
