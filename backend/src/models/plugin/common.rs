use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArgType {
    ListFromData,
    String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ArgDef {
    pub name: String,
    #[serde(default = "arg_type_default")]
    pub arg_type: ArgType,
    pub value: String,
    #[serde(default)]
    pub data_id: Option<String>,
}

impl Hash for ArgDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ArgDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Script {
    pub script_type: String,
    pub script: String,
}

fn arg_type_default() -> ArgType {
    ArgType::String
}
