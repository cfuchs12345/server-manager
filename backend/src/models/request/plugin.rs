use serde::{Deserialize, Serialize};

use super::common::QueryParam;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum PluginsActionType {
    Disable,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PluginsAction {
    pub action_type: PluginsActionType,
    pub params: Vec<QueryParam>,
}
