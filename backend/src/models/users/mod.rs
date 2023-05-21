use serde::{Serialize, Deserialize};

use crate::common;

use super::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    user_id: String,
    full_name: String,
    email: String,
    #[serde(default)]
    password_hash: String,
}

impl User {
    pub fn new(user_id: String, full_name: String, email: String, password_hash: String) -> Self {
        User {
            user_id,
            full_name,
            email,
            password_hash
        }
    }

    pub fn update_password_hash( &mut self, password_hash: String) {
        self.password_hash = password_hash;
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn check_password(&self, password_to_check: &str) -> Result<bool, AppError> {
        common::verify_password(password_to_check, self.password_hash.as_str())
    }
}