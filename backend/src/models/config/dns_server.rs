use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct DNSServer {
    pub ipaddress: IpAddr,
    pub port: u16,
}
