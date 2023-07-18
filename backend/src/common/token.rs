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
        let token = generate_unique_token().await;

        insert_token(&token).await;

        Ok(Token { token })
    }

    pub async fn is_valid(&self, remove_if_exists: bool) -> bool {
        let exists = token_exists(&self.token).await;

        if exists && remove_if_exists {
            remove_token(&self.token).await;
        }
        exists
    }
}

async fn generate_unique_token() -> String {
    let read_tokens = GENERATED_TOKENS.read().await;
    let mut token;
    'outer: loop {
        token = super::generate_short_random_string();

        if !read_tokens.contains_key(&token) {
            break 'outer;
        }
    }
    token
}

async fn token_exists(token: &str) -> bool {
    let read_tokens = GENERATED_TOKENS.read().await;
    read_tokens.contains_key(token)
}

async fn insert_token(token: &str) {
    let mut write_keys = GENERATED_TOKENS.write().await;
    write_keys.insert(token.to_owned(), token.to_owned());
}

pub async fn remove_token(token: &str) {
    let mut write_tokens = GENERATED_TOKENS.write().await;

    write_tokens.remove(token);
}
