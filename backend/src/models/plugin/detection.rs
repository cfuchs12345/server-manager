use serde::{Deserialize, Serialize};

use super::{
    common::{ArgDef, Script},
    ParamDef,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DetectionEntry {
    #[serde(default)]
    pub params: Vec<ParamDef>,
    #[serde(default)]
    pub args: Vec<ArgDef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DetectionDef {
    #[serde(default)]
    pub list: Vec<DetectionEntry>,
    #[serde(default)]
    pub script: Script,
    #[serde(default)]
    pub detection_possible: bool,
    #[serde(default = "default_http")]
    pub command: String,
}

fn default_http() -> String {
    "http".to_owned()
}
