use serde::{Deserialize, Serialize};

use super::common::Script;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DetectionEntry {
    #[serde(default)]
    pub defaultports: Vec<u16>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    #[serde(default)]
    pub list: Vec<DetectionEntry>,
    #[serde(default)]
    pub script: Script,
    #[serde(default)]
    pub detection_possible: bool,
}
