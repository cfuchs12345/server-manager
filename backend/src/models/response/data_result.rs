

use std::net::{IpAddr, Ipv4Addr};

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub result: String,
    pub check_results: Vec<ConditionCheckResult>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConditionCheckResult {
    #[serde(default = "default_ipaddress")]
    pub ipaddress: IpAddr,
    pub feature_id: String,
    pub action_id: String,
    pub action_params: String,
    pub result: bool
}

impl ConditionCheckResult {
    pub fn get_key(self) -> String {
        format!("{}_{}_{}", self.ipaddress, self.feature_id, self.action_id)
    }
}

fn default_ipaddress() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
}



mod test {
    
}