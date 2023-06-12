use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};

use rand::Rng;
use tokio::sync::RwLock;

use crate::models::error::AppError;

lazy_static! {
    static ref GENERATED_KEYS: RwLock<HashMap<u32, (NaiveDateTime, String)>> =
        RwLock::new(HashMap::new());
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct OneTimeKey {
    id: u32,
    key: String,
}

impl OneTimeKey {
    pub async fn generate() -> Result<Self, AppError> {
        let mut rng = rand::thread_rng();

        let mut id = rng.gen::<u32>();
        let read_keys = GENERATED_KEYS.read().await;

        'outer: loop {
            if !read_keys.contains_key(&id) {
                break 'outer;
            } else {
                id = rng.gen::<u32>();
            }
        }
        let key = super::generate_short_random_string();

        drop(read_keys);

        let mut write_keys = GENERATED_KEYS.write().await;

        write_keys.insert(id, (Utc::now().naive_utc(), key.clone()));
        Ok(OneTimeKey { id, key })
    }

    pub async fn get_one_time_key(id: u32) -> Result<(NaiveDateTime, String), AppError> {
        let read_keys = GENERATED_KEYS.read().await;

        let found = read_keys.get(&id).map(|val| val.to_owned());

        found.ok_or(AppError::Unknown("token does not exist".to_owned()))
    }
}

pub async fn invalidate_expired_one_time_keys() -> Result<(), AppError> {
    let now = Utc::now().naive_utc().timestamp();
    let mut map = GENERATED_KEYS.write().await;

    map.retain(|_, v| now - v.0.timestamp() < 30 * 5);

    Ok(())
}
