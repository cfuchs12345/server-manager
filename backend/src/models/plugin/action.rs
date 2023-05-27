use serde::{Deserialize, Serialize};

use super::common::ArgDef;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Active,
    Inactive,
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Action {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_true")]
    pub show_on_main: bool,
    #[serde(default)]
    pub depends: Vec<DependsDef>,
    #[serde(default)]
    pub available_for_state: State,
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

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DependsDef {
    pub data_id: String,
    #[serde(default)]
    pub script_type: String,
    #[serde(default)]
    pub script: String,
}

fn default_true() -> bool {
    true
}
