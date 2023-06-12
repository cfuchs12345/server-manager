use actix_session::Session;
use serde::{Deserialize, Serialize};

use crate::models::error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ClientKey {
    pub key: String,
}

const SESSION_KEY: &str = "client_key";

impl ClientKey {
    pub fn new() -> Self {
        ClientKey {
            key: super::generate_short_random_string(),
        }
    }

    pub fn register_for_session(&self, session: Session) -> Result<Self, AppError> {
        if Self::get(&session)?.is_some() {
            log::warn!("session key {} already exists", SESSION_KEY);
        }
        session.insert(SESSION_KEY, self).map_err(|err| {
            AppError::Unknown(format!(
                "Could not insert value with key {} into session due to error: {}",
                SESSION_KEY, err
            ))
        })?;
        Ok(self.to_owned())
    }

    pub fn get_from_session(session: Session) -> Result<Option<ClientKey>, AppError> {
        Self::get(&session)
    }

    fn get(session: &Session) -> Result<Option<ClientKey>, AppError> {
        session.get::<ClientKey>(SESSION_KEY).map_err(|err| {
            AppError::Unknown(format!(
                "error while getting value for session key {}. Error was was {}",
                SESSION_KEY, err
            ))
        })
    }
}
