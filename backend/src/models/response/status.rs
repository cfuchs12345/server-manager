use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Status {
    pub is_running: bool,
    pub ipaddress: String,
}


impl Status {
    pub fn new(ipaddress: String) -> Self {
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

