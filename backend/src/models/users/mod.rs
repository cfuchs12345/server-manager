use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    common,
    event_handling::{EventSource, ObjectType, Value},
};

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
    #[allow(dead_code)]
    pub fn new(user_id: String, full_name: String, email: String, password_hash: String) -> Self {
        User {
            user_id,
            full_name,
            email,
            password_hash,
        }
    }

    pub fn copy_no_passwd(&self) -> Self {
        User {
            password_hash: "".to_owned(),
            ..self.clone()
        }
    }

    pub fn update_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
    }

    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
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

impl EventSource for User {
    fn get_object_type(&self) -> ObjectType {
        ObjectType::User
    }

    fn get_event_key_name(&self) -> String {
        "user_id".to_string()
    }

    fn get_event_key(&self) -> String {
        self.user_id.to_owned()
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        Ok(self.user_id.to_owned())
    }

    fn get_key_values(&self) -> HashMap<String, Value> {
        let mut kv = HashMap::new();
        kv.insert(
            "password_hash".to_owned(),
            Value::String("password_hash".to_owned()),
        );
        kv.insert("email".to_owned(), Value::String("email".to_owned()));
        kv.insert(
            "full_name".to_owned(),
            Value::String("full_name".to_owned()),
        );
        kv
    }
}
