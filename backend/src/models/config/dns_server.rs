
use std::net::IpAddr;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct DNSServer {
    pub ipaddress: IpAddr,
    pub port: u16
}