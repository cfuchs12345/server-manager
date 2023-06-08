use serde::{Deserialize, Serialize};

use super::{
    common::{ArgDef, Script},
    monitoring::Monitioring,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_true")]
    pub output: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub result_format: ResultFormat,
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
    #[serde(default)]
    pub post_process: Option<Script>,
    #[serde(default)]
    pub monitoring: Vec<Monitioring>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ResultFormat {
    #[default]
    JSON,
    XML,
}

fn default_true() -> bool {
    true
}
