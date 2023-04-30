use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
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


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Script {
    pub script_type: String,
    pub script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectionEntry {
    #[serde(default)]
    pub defaultports: Vec<u16>,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Detection {
    #[serde(default)]
    pub list: Vec<DetectionEntry>,
    #[serde(default)]
    pub script: Script,
    #[serde(default)]
    pub detection_possible: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum State {
    Active,
    Inactive,
    #[default]
    Any
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub depends: Vec<DependsDef>,
    #[serde(default)]
    pub available_for_state : State,
    #[serde(default = "default_true")]
    pub needs_confirmation: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Vec<ArgDef>,
}

fn default_true() -> bool {
    true
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependsDef {
    pub data_id: String,
    #[serde(default)]
    pub script_type: String,
    #[serde(default)]
    pub script: String
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ParamDef {
    pub name: String,
    pub param_type: String,
    pub default_value: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CredentialDef {
    pub name: String,
    pub credential_type: String,
    pub encrypt: bool,
    pub default_value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArgDef {
    pub arg_type: String,
    pub value: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub template_helper_script: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Vec<ArgDef>,
}
