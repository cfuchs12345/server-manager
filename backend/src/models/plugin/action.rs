use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::common::{ArgDef, Script};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Active,
    Inactive,
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ActionDef {
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

impl Hash for ActionDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.show_on_main.hash(state);
        self.needs_confirmation.hash(state);
        self.available_for_state.hash(state);
    }
}

impl PartialEq for ActionDef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.show_on_main == other.show_on_main
            && self.needs_confirmation == other.needs_confirmation
            && self.available_for_state == other.available_for_state
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependsDef {
    pub data_id: String,
    #[serde(default)]
    pub script: Script,
}

fn default_true() -> bool {
    true
}
