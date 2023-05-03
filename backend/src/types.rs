use serde::{Deserialize, Serialize};

use std::collections::HashMap;


use crate::{plugin_types::{ParamDef, ArgDef, Plugin, Data, Action}, server_types::{Credential, Feature, Param}, persistence::Persistence};

pub struct QueryParamsAsMap {
    params: HashMap<String, String>
}

impl From<Vec<QueryParam>> for QueryParamsAsMap {
    fn from(input_params: Vec<QueryParam>) -> Self {
        QueryParamsAsMap {
            params: input_params.iter().map( |param| (param.name.clone(), param.value.clone())).collect()
        }
    }    
}

impl QueryParamsAsMap {
    pub fn get(&self, param: &str) -> Option<&String> {
        self.params.get(param)
    }

    pub fn get_split_by(&self, param: &str, split: &str) -> Option<Vec<String>> {
        match self.params.get(param) {
            Some(value) => {
                let res: Vec<String> = value.split(split).map(str::to_string).collect();
                Some(res)
            }
            None => None
        }
    }
}


#[derive (Debug)]
pub struct ActionOrDataInput<'a> {
    pub command: String,
    args: Vec<ArgDef>,
    params: Vec<Param>,
    default_params: Vec<ParamDef>,
    credentials: Vec<Credential>,
    pub accept_self_signed_ceritificates: bool,
    pub persistence: &'a Persistence
}
impl ActionOrDataInput<'_> {
    pub fn get_input_from_action<'a>(action: &Action, plugin: &Plugin, feature: &Feature, accept_self_signed_ceritificates: bool, persistence: &'a Persistence) -> ActionOrDataInput<'a> {
        ActionOrDataInput{
            command: action.command.clone(),
            args: action.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            credentials: feature.credentials.clone(),
            accept_self_signed_ceritificates,
            persistence
        }
    }

    pub fn get_input_from_data<'a>(data: &Data, plugin: &Plugin, feature: &Feature, accept_self_signed_ceritificates: bool, persistence: &'a Persistence) ->  ActionOrDataInput<'a> {
        ActionOrDataInput{
            command: data.command.clone(),
            args: data.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            credentials: feature.credentials.clone(),
            accept_self_signed_ceritificates,
            persistence
        }
    }

    pub fn find_param(&self, param_name: &str) -> Option<&Param> {
        self.params.iter().find( |param| param.name == param_name)
    }

    pub fn find_default_param(&self, param_name: &str) -> Option<&ParamDef> {
        self.default_params.iter().find( |param| param.name == param_name)
    }

    pub fn find_credential(&self, credential_name: &str) -> Option<&Credential> {
        self.credentials.iter().find( |credential| credential.name == credential_name)
    }
    
    pub fn find_arg(&self, arg_type: &str) ->  Option<&ArgDef> {
        self.args.iter().find(|argdef| argdef.arg_type == arg_type)        
    }

    pub fn find_all_args(&self, arg_type: &str) ->  Vec<&ArgDef> {
        self.args.iter().filter(|argdef| argdef.arg_type == arg_type).collect()        
    }
}




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
pub struct QueryParam {
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
    pub params: Vec<QueryParam>
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
    pub params: Vec<QueryParam>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServersAction {
    pub action_type: ServersActionType,
    pub params: Vec<QueryParam>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerAction {
    pub action_type: ServerActionType,
    pub params: Vec<QueryParam>
}
