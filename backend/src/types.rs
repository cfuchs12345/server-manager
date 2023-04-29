use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct HostInformation {
    pub ipaddress: String,
    pub is_running: bool,
    pub dnsname: String,
}

impl PartialEq for HostInformation {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Status {
    pub is_running: bool,
    pub ipaddress: String,
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}



#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ServersActionType {
    Status,
    FeatureScan
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ServerActionType {
    FeatureScan,
    Status,
    ExecuteFeatureAction,
    ActionConditionCheck,
    QueryDependencyData,
    QueryData,
    IsConditionForFeatureActionMet,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum NetworkActionType {
    AutoDiscover
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum PluginsActionType {
    Disable
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PluginsAction {
    pub action_type: PluginsActionType,
    pub params: Vec<Param>
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
    pub params: Vec<Param>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServersAction {
    pub action_type: ServersActionType,
    pub params: Vec<Param>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerAction {
    pub action_type: ServerActionType,
    pub params: Vec<Param>
}

