use serde::{Deserialize, Serialize};

use super::common::Script;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub enum Level {
    #[default]
    Info,
    Warn,
    Error,
    Critical,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NotificationDef {
    pub id: String,
    pub name: String,
    pub data_id: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub script: Script,
    #[serde(default)]
    pub notification_level: Level,
}

impl NotificationDef {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub ipaddress: String,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub notification_level: Level,
}
