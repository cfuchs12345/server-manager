use serde::{Deserialize, Serialize};

use super::common::QueryParam;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ServersActionType {
    Status,
    FeatureScan,
    ActionConditionCheck,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ServerActionType {
    FeatureScan,
    Status,
    ExecuteFeatureAction,
    QueryData,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum NetworkActionType {
    AutoDiscover,
}

/**
 *
 *
 * Discovery Query:
 * "action_type", AutoDiscover
 * "network" : "xxx.xxx.xxx.xxx/xx",
 * "lookup_names": "true/false",
 * "dns_servers": "xxx.xxx.xxx.xxx,yyy.yyy.yyy.yyy..."
 */
#[derive(Deserialize, Serialize, Clone)]
pub struct NetworksAction {
    pub action_type: NetworkActionType,
    pub params: Vec<QueryParam>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServersAction {
    pub action_type: ServersActionType,
    pub params: Vec<QueryParam>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerAction {
    pub action_type: ServerActionType,
    pub params: Vec<QueryParam>,
}
