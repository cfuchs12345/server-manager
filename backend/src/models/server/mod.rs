use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Server {
    pub ipaddress: IpAddr,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dnsname: String,
    #[serde(default)]
    pub features: Vec<Feature>,
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}

impl Server {
    pub fn find_feature(&self, feature_id: &str) -> Option<Feature> {
        self.features.iter().find(|f| f.id == feature_id).cloned()
    }

    pub fn new_only_ip(ipaddress: IpAddr) -> Self {
        Server {
            ipaddress,
            name: "".to_owned(),
            dnsname: "".to_owned(),
            features: Vec::new(),
        }
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

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Feature {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(default)]
    pub credentials: Vec<Credential>,
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
