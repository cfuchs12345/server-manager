use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
    net::IpAddr,
};

use serde::{Deserialize, Serialize};

use crate::{
    common,
    event_handling::{EventSource, ObjectType, Value},
};

use super::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Server {
    ipaddress: IpAddr,
    #[serde(default)]
    name: String,
    #[serde(default)]
    dnsname: String,
    #[serde(default)]
    features: Vec<Feature>,
    #[serde(default)]
    version: i64,
}

impl Hash for Server {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ipaddress.hash(state);
        self.name.hash(state);
        self.dnsname.hash(state);
        self.features.hash(state);
    }
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
            && self.name == other.name
            && self.dnsname == other.dnsname
            && self.features == other.features
    }
}

impl Server {
    pub fn new_only_ip(ipaddress: IpAddr) -> Self {
        Server {
            ipaddress,
            name: "".to_owned(),
            dnsname: "".to_owned(),
            features: Vec::new(),
            version: -1,
        }
    }

    pub fn new(ipaddress: IpAddr, name: String, dnsname: String, features: Vec<Feature>) -> Self {
        Server {
            ipaddress,
            name,
            dnsname,
            features,
            version: -1,
        }
    }

    pub fn get_ipaddress(&self) -> IpAddr {
        self.ipaddress
    }

    pub fn get_features(&self) -> Vec<Feature> {
        self.features.clone()
    }

    pub fn set_features(&mut self, features: Vec<Feature>) {
        self.features = features
    }

    pub fn find_feature(&self, feature_id: &str) -> Option<Feature> {
        self.features.iter().find(|f| f.id == feature_id).cloned()
    }

    pub fn replace_feature(mut self, new_feature: Feature) -> Self {
        if let Some(index) = self.features.iter().position(|f| f.id == new_feature.id) {
            self.features.push(new_feature.clone());
            let old_feature = self.features.swap_remove(index);
            log::debug!(
                "replaced old feature {:?} with new {:?} at pos {}",
                old_feature,
                new_feature,
                index
            );
        } else {
            log::error!("did not find feature to replace {:?}", new_feature);
        }
        self
    }
}

impl EventSource for Server {
    fn get_object_type(&self) -> ObjectType {
        ObjectType::Server
    }

    fn get_event_key_name(&self) -> String {
        common::IPADDRESS.to_owned()
    }

    fn get_event_key(&self) -> String {
        format!("{:?}", self.ipaddress)
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        serde_json::to_string(self).map_err(AppError::from)
    }

    fn get_version(&self) -> i64 {
        self.version
    }

    fn get_key_values(&self) -> HashMap<String, Value> {
        let mut kv = HashMap::new();
        kv.insert("name".to_owned(), Value::String(self.name.clone()));
        kv.insert("dnsname".to_owned(), Value::String(self.dnsname.clone()));
        kv.insert(
            "features".to_owned(),
            Value::String(format!("{:?}", self.features)),
        );
        kv
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Eq)]
pub struct Feature {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(default)]
    pub credentials: Vec<Credential>,
}

impl Hash for Feature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Feature").field("id", &self.id).finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Param {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Credential {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub encrypted: bool,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct FeaturesOfServer {
    pub ipaddress: IpAddr,
    pub features: Vec<Feature>,
}

impl PartialEq for FeaturesOfServer {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}
