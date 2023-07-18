use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use tokio::sync::RwLock;

use crate::models::error::AppError;

lazy_static! {
    static ref GENERATED_TOKENS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Token {
    token: String,
}

impl From<String> for Token {
    fn from(string: String) -> Self {
        Token::new(string)
    }
}

impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }

    pub async fn generate() -> Result<Self, AppError> {
        let read_tokens = GENERATED_TOKENS.read().await;

        let mut token;
        'outer: loop {
            token = super::generate_short_random_string();

            if !read_tokens.contains_key(&token) {
                break 'outer;
            }
        }

        drop(read_tokens);

        let mut write_keys = GENERATED_TOKENS.write().await;

        write_keys.insert(token.clone(), token.clone());
        Ok(Token { token })
    }

    pub async fn is_valid(&self, remove_if_valid: bool) -> bool {
        let read_tokens = GENERATED_TOKENS.read().await;
        let result = read_tokens.contains_key(&self.token);
        if result && remove_if_valid {
            self.remove_token().await;
        }
        result
    }

    pub async fn remove_token(&self) {
        let mut write_tokens = GENERATED_TOKENS.write().await;

        write_tokens.remove(&self.token);
    }
}
