use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserToken {
    pub user_id: String,
    pub token: String,
    pub client_key: String,
}
