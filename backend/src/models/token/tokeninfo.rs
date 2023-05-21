use sqlx::types::chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenInfo {
    creation_date: NaiveDateTime,
}

const TOKEN_EXPIRY_DURATION_SECS: i64 = 3600; // 1 hour

impl TokenInfo {
    pub fn new() -> Self {
        TokenInfo {
            creation_date: Utc::now().naive_utc(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now().naive_utc().timestamp();
        self.creation_date.timestamp() - now > TOKEN_EXPIRY_DURATION_SECS
    }
}
