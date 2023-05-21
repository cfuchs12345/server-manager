use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use sqlx::{types::chrono::{NaiveDateTime, Utc}};

use rand::Rng;
use tokio::sync::RwLock;




lazy_static!{
    static ref GENERATED_KEYS:  RwLock<HashMap<u32, (NaiveDateTime, String)>> = RwLock::new(HashMap::new());
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct OneTimeKey {
    id: u32,
    key: String
}

impl OneTimeKey {  
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        let mut id = rng.gen::<u32>();
        'outer: loop {
            if !GENERATED_KEYS.try_read().unwrap().contains_key(&id) {
                break 'outer;
            }
            else {
                id = rng.gen::<u32>();
            }
        }
        let key = super::generate_short_random_string();

        GENERATED_KEYS.try_write().unwrap().insert(id, (Utc::now().naive_utc(), key.clone()));
        OneTimeKey { id,  key}
    }

    pub fn invalidate(&self) {
        GENERATED_KEYS.try_write().unwrap().remove(&self.id);
    }

    pub fn invalidate_expired(&self) {
        let now = Utc::now().naive_utc().timestamp();
        let map = GENERATED_KEYS.try_write().unwrap();

        for val in map.values() {
            if now - val.0.timestamp() > 30 * 5 {
                self.invalidate()
            }
        }
    }

    pub fn get_token(id: u32) -> Option<(NaiveDateTime, String)>{
        GENERATED_KEYS.try_read().unwrap().get(&id).map(|val| val.to_owned())
    }
}

