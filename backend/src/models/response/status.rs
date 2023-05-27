use std::net::IpAddr;

use serde::{Serialize, Deserialize};

use crate::commands::ping::PingCommandResult;

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Status {
    pub is_running: bool,
    pub ipaddress: IpAddr,
}


impl Status {
    pub fn new(ipaddress: IpAddr) -> Self {
        Status {
            is_running: false,
            ipaddress
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
        Status {
            is_running: res.get_result(),
            ipaddress: res.get_ipaddress()
        }
    }
}

