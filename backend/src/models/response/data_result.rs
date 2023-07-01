use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::IPADDRESS,
    event_handling::{EventSource, ObjectType, Value},
    models::error::AppError,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub result: String,
    pub check_results: Vec<ConditionCheckResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConditionCheckResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub subresults: Vec<ConditionCheckSubResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConditionCheckSubResult {
    pub feature_id: String,
    pub action_id: String,
    pub action_params: String,
    pub result: bool,
}

impl ConditionCheckResult {
    pub fn get_key(self) -> String {
        format!("{}", self.ipaddress)
    }
}

impl EventSource for ConditionCheckResult {
    fn get_object_type(&self) -> ObjectType {
        ObjectType::ConditionCheckResult
    }

    fn get_event_key_name(&self) -> String {
        IPADDRESS.to_owned()
    }

    fn get_event_key(&self) -> String {
        format!("{:?}", self.ipaddress)
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        serde_json::to_string(self).map_err(AppError::from)
    }

    fn get_key_values(&self) -> HashMap<String, Value> {
        let mut kv = HashMap::new();
        kv.insert(
            "value".to_string(),
            Value::String(
                serde_json::to_string(self)
                    .map_err(AppError::from)
                    .expect(""),
            ),
        );
        kv
    }
}

fn default_ipaddress() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
}

mod test {}
