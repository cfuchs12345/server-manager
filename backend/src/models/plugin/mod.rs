pub mod action;
pub mod common;
pub mod data;
pub mod detection;
pub mod monitoring;
pub mod notification;
pub mod sub_action;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::common::hash_as_string;
use crate::event_handling::{EventSource, ObjectType};
use std::hash::{Hash, Hasher};

use self::{
    action::ActionDef, data::DataDef, detection::DetectionDef, notification::NotificationDef,
};

use super::error::AppError;

#[derive(Serialize, Deserialize, Clone, Eq)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub server_icon: String,
    pub detection: DetectionDef,
    #[serde(default)]
    pub credentials: Vec<CredentialDef>,
    #[serde(default)]
    pub params: Vec<ParamDef>,
    #[serde(default)]
    pub data: Vec<DataDef>,
    #[serde(default)]
    pub notifications: Vec<NotificationDef>,
    #[serde(default)]
    pub actions: Vec<ActionDef>,
}

impl Hash for Plugin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.description.hash(state);
        self.actions.hash(state);
    }
}

impl Plugin {
    pub fn find_action(&self, action_id: &str) -> Option<&ActionDef> {
        self.actions.iter().find(|plugin| plugin.id == action_id)
    }
}

impl Debug for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl EventSource for Plugin {
    fn get_object_type(&self) -> ObjectType {
        ObjectType::Plugin
    }

    fn get_event_key_name(&self) -> String {
        "id".to_owned()
    }

    fn get_event_key(&self) -> String {
        self.id.to_owned()
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        Ok("".to_owned())
    }

    fn get_change_flag(&self) -> String {
        hash_as_string(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ParamDef {
    pub name: String,
    pub param_type: String,
    pub default_value: String,
    #[serde(default = "default_false")]
    pub mandatory: bool,
}

impl Hash for ParamDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ParamDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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
