
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct DNSServer {
    pub ipaddress: String,
    pub port: u16
}