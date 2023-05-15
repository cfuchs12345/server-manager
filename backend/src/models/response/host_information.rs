use serde::{Deserialize, Serialize};

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