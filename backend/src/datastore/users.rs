
use crate::models::{error::AppError, users::User};

use super::{persistence::Persistence, Entry};


const TABLE: &str = "users";

fn entry_to_user(entry: &Entry) -> User {
    serde_json::from_str(entry.value.as_str()).unwrap()
}

fn entries_to_users(jsons: Vec<Entry>) -> Vec<User> {
    jsons.iter().map( entry_to_user ).collect()
}

fn user_to_entry(user: &User) -> Entry {
    Entry {
        key: user.get_user_id(),
        value: serde_json::to_string(user).unwrap()
    }    
}

pub async fn insert_user(persistence: &Persistence, user: &User) -> Result<bool, AppError> {
    let result = persistence.insert(TABLE, user_to_entry(user)).await?;

    Ok(result > 0)
}


pub async fn update_user(persistence: &Persistence, user: &User) -> Result<bool, AppError> {
    let result = persistence.update(TABLE, user_to_entry(user)).await.unwrap();

    Ok(result > 0)
}

pub async fn delete_user(persistence: &Persistence, user_id: &str) -> Result<bool, AppError> {
    let result = persistence.delete(TABLE, user_id).await.unwrap();

    Ok(result > 0)
}

pub async fn load_all_users(persistence: &Persistence) -> Result<Vec<User>, AppError> {
        let user_entries = persistence.get_all(TABLE, Some("key")).await.unwrap();

        Ok(entries_to_users(user_entries))
}

pub async fn get_user(persistence: &Persistence, user_id: &str) -> Result<User, AppError> {
    match persistence.get(TABLE, user_id).await? {
        Some(entry) => {
            Ok(entry_to_user(&entry))
        },
        None => {
            Err(AppError::UserNotFound(user_id.to_owned()))
        }
    }
}