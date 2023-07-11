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

#[derive(Deserialize, Serialize, Clone, Debug, Hash)]
pub struct DataResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub data_id: String,
    pub result: String,
    pub check_results: Vec<ConditionCheckResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Hash)]
pub struct ConditionCheckResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub data_id: String,
    pub subresults: Vec<ConditionCheckSubResult>,
    pub version: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug, Hash)]
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
        format!("{}_{}", IPADDRESS, "data_id")
    }

    fn get_event_key(&self) -> String {
        format!("{:?}_{:?}", self.ipaddress, self.data_id)
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        serde_json::to_string(self).map_err(AppError::from)
    }

    fn get_version(&self) -> i64 {
        self.version
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
