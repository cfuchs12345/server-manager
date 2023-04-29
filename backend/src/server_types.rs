use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Server {
    pub ipaddress: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dnsname: String,
    #[serde(default)]
    pub features: Vec<Feature>
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
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
    pub credentials: Vec<Credential>
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
    pub value: String
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
    pub value: String    
}


#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct FeaturesOfServer {
    pub ipaddress: String,
    pub features: Vec<Feature>,
}

impl PartialEq for FeaturesOfServer {
    fn eq(&self, other: &Self) -> bool {
        self.ipaddress == other.ipaddress
    }
}