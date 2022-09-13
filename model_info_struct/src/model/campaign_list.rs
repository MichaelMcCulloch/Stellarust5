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
        match self
            .list
            .binary_search_by_key(&game_data.campaign_name.as_str(), |m| {
                m.campaign_name.as_str()
            }) {
            Ok(pos) => {
                self.list.remove(pos);
                self.list.insert(pos, CampaignInfoStruct::from(game_data));
            }
            Err(pos) => {
                self.list.insert(pos, CampaignInfoStruct::from(game_data));
            }
        };
        Some(self.list.clone())
    }

    fn update_all(
        &mut self,
        game_data_history: &HashMap<String, Vec<ModelDataPoint>>,
    ) -> Option<Self::Representation> {
        self.list = game_data_history
            .iter()
            .map(|(_, v)| CampaignInfoStruct::from(v.last().unwrap()))
            .collect::<Vec<_>>();
        Some(self.list.clone())
    }
}

impl From<ModelDataPoint> for CampaignInfoStruct {
    fn from(model_data_point: ModelDataPoint) -> Self {
        CampaignInfoStruct::from(&model_data_point)
    }
}

impl From<&ModelDataPoint> for CampaignInfoStruct {
    fn from(model_data_point: &ModelDataPoint) -> Self {
        CampaignInfoStruct {
            campaign_name: model_data_point.campaign_name.clone(),
            empire_list: model_data_point
                .empires
                .iter()
                .map(|empire_data| Empire {
                    name: empire_data.name.clone(),
                    player: match empire_data.driver.clone() {
                        game_data_info_struct::PlayerClass::Human(name) => Some(name),
                        game_data_info_struct::PlayerClass::Computer => None,
                    },
                })
                .collect(),
        }
    }
}

impl ModelSpec for CampaignListModelSpec {
    type Model = CampaignListModel;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Empire {
    pub name: String,
    pub player: Option<String>,
}

// Clone is only used to make an ownable copy to serialize and send.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignInfoStruct {
    pub campaign_name: String,
    pub empire_list: Vec<Empire>,
}
