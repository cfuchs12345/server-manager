use std::{collections::HashMap, net::IpAddr};

use serde::{Deserialize, Serialize};

use crate::{
    commands::ping::PingCommandResult,
    common,
    event_handling::{EventSource, ObjectType, Value},
    models::error::AppError,
};

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct Status {
    pub is_running: bool,
    pub ipaddress: IpAddr,
    pub error: bool,
}

impl Status {
    pub fn new(ipaddress: IpAddr) -> Self {
        Status {
            is_running: false,
            error: false,
            ipaddress,
        }
    }

    pub fn new_with_running(ipaddress: IpAddr, is_running: bool) -> Self {
        Status {
            is_running,
            error: false,
            ipaddress,
        }
    }

    pub fn error(ipaddress: IpAddr) -> Self {
        Status {
            is_running: false,
            error: true,
            ipaddress,
        }
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}

impl From<PingCommandResult> for Status {
    fn from(res: PingCommandResult) -> Self {
        Status::new_with_running(res.get_ipaddress(), res.get_result())
    }
}

impl EventSource for Status {
    fn get_object_type(&self) -> ObjectType {
        ObjectType::Status
    }

    fn get_event_key_name(&self) -> String {
        common::IPADDRESS.to_owned()
    }

    fn get_event_key(&self) -> String {
        self.ipaddress.to_string()
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        serde_json::to_string(self).map_err(AppError::from)
    }

    fn get_key_values(&self) -> std::collections::HashMap<String, Value> {
        let mut kv = HashMap::new();
        kv.insert("is_running".to_owned(), Value::Boolean(self.is_running));
        kv
    }
}
