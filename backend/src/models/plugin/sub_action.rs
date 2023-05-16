use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq,)]
pub struct SubAction {
    pub feature_id: Option<String>,
    pub action_id: Option<String>,
    #[serde(default)]
    pub action_params: Option<String>,
    #[serde(default)]
    pub action_image: Option<String>,
}

impl From<String> for SubAction {
    fn from(value: String) -> Self {
        let stripped_value = value.replace("[[Action ", "").replace("]]", "").replace('\"', "");

        let map:HashMap<String,String> = map_data(stripped_value);
      
        SubAction {
            feature_id:  map.get("feature.id").map(|v| v.to_owned()),
            action_id: map.get("action.id").map(|v| v.to_owned()),
            action_params:  map.get("action.params").map(|v| v.to_owned()),
            action_image:  map.get("action.image").map(|v| v.to_owned())
         }
    }
}

fn map_data(input: String) -> HashMap<String,String> {
    input.split_whitespace().map(|s| s.split_at(s.find('=').unwrap())).map(|(key, val)| (key, &val[1..])).map(|(key, val)| (key.to_owned(), val.to_owned())).collect()    
}

