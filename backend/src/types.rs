use serde::{Deserialize, Serialize};

use std::collections::HashMap;


use crate::{plugin_types::{ParamDef, ArgDef, Plugin, Data, Action}, server_types::{Credential, Feature, Param, Server}, inmemory};

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

    pub fn get_as_str(&self, param: &str) -> Option<&str> {
        self.params.get(param).map(|value| value.as_str())
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
pub struct ActionOrDataInput {
    pub command: String,
    args: Vec<ArgDef>,
    params: Vec<Param>,
    default_params: Vec<ParamDef>,
    action_params: Vec<Param>,
    credentials: Vec<Credential>,
    pub crypto_key: String
}
impl ActionOrDataInput {
    pub fn get_input_from_action(action: &Action,  action_params: Option<&str>, plugin: &Plugin, feature: &Feature, crypto_key: String ) -> ActionOrDataInput {
        let action_params = to_params(action_params);
        ActionOrDataInput{
            command: action.command.clone(),
            args: action.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            action_params,
            credentials: feature.credentials.clone(),
            crypto_key
        }
    }

    pub fn get_input_from_data(data: &Data, action_params: Option<&str>, plugin: &Plugin, feature: &Feature, crypto_key: &str) ->  ActionOrDataInput {
        let action_params = to_params(action_params);

        ActionOrDataInput{
            command: data.command.clone(),
            args: data.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            action_params,
            credentials: feature.credentials.clone(),
            crypto_key: crypto_key.to_owned()
        }
    }

    pub fn find_param(&self, param_name: &str) -> Option<&Param> {
        let from_feature = self.params.iter().find( |param| param.name == param_name);

        if from_feature.is_some() {
            from_feature
        }
        else { // there are also parameter coming with the request i.e. for sub actions. also check these...
            self.action_params.iter().find(|param| param.name == param_name)
        }
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

fn to_params(action_params: Option<&str>) -> Vec<Param> {
    if action_params.is_none() {
        return Vec::new();
    }
    let mut list = Vec::new();

    let split = action_params.unwrap().split(",");

    for str in split {
        let single_param = str.split_at(str.find("=").unwrap());
        
        list.push(Param {
            name: single_param.0.to_owned(),
            value: single_param.1[1..].to_owned() // skip the first char which is still the separator
        });
    }

    list
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

impl Status {
    pub fn new(ipaddress: String) -> Self {
        Status {
            is_running: false,
            ipaddress
        }
    }
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
    FeatureScan,
    ActionConditionCheck
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

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ConditionCheckResult {
    pub ipaddress: String,
    pub feature_id: String,
    pub action_id: String,
    pub action_params: String,
    pub result: bool
}

impl ConditionCheckResult {
    pub fn get_key(self) -> String {
        format!("{}_{}_{}", self.ipaddress, self.feature_id, self.action_id)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataResult {
    pub ipaddress: String,
    pub result: String,
    pub check_results: Vec<ConditionCheckResult>
}