use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PasswordChange {
    pub old_password: String,
    pub new_password: String
}