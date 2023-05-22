pub mod data;
pub mod detection;
pub mod action;
pub mod common;
pub mod sub_action;

use serde::{Serialize, Deserialize};

use self::{data::Data, action::Action, detection::Detection};


#[derive(Serialize, Deserialize, Debug, Clone, Eq,)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub server_icon: String,
    pub detection: Detection,
    #[serde(default)]
    pub credentials: Vec<CredentialDef>,
    #[serde(default)]
    pub params: Vec<ParamDef>,
    #[serde(default)]
    pub data: Vec<Data>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Plugin {
    pub fn find_action(&self, action_id: &str) -> Option<&Action> {
        self.actions.iter().find( |plugin| plugin.id == action_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq,)]
pub struct ParamDef {
    pub name: String,
    pub param_type: String,
    pub default_value: String,
    #[serde(default = "default_false")]
    pub mandatory: bool,
}

impl PartialEq for ParamDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq,)]
pub struct CredentialDef {
    pub name: String,
    pub credential_type: String,
    pub encrypt: bool,
    pub default_value: String,
    #[serde(default = "default_false")]
    pub mandatory: bool,
}



fn default_false() -> bool {
    false
}