use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataResult {
    pub ipaddress: String,
    pub result: String,
    pub check_results: Vec<ConditionCheckResult>
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ConditionCheckResult {
    pub ipaddress: String,
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

