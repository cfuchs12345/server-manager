use serde::{Deserialize, Serialize};

use super::common::{ArgDef, Script};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SeriesType {
    datetime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ChartyType {
    bar,
    line,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Monitioring {
    pub pre_process: Option<Script>,
    pub id: String,
    pub name: String,
    pub series_type: SeriesType,
    pub chart_type: ChartyType,
    pub identifier: KeyValue,
    pub sub_identifier: Option<KeyValue>,
    pub value: KeyValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub name: String,
    pub value_type: String,
    pub value: String,
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
