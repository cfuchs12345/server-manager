use crate::models::{error::AppError, users::User};

use super::{persistence::Persistence, Entry};

const TABLE: &str = "users";

fn entry_to_user(entry: &Entry) -> Result<User, AppError> {
    serde_json::from_str(entry.value.as_str()).map_err(AppError::from)
}

fn entries_to_users(entries: Vec<Entry>) -> Result<Vec<User>, AppError> {
    let mut list = Vec::new();

    for entry in entries {
        list.push(entry_to_user(&entry)?);
    }
    Ok(list)
}

fn user_to_entry(user: &User) -> Result<Entry, AppError> {
    Ok(Entry {
        key: user.get_user_id(),
        value: serde_json::to_string(user)?,
    })
}

pub async fn insert_user(persistence: &Persistence, user: &User) -> Result<bool, AppError> {
    let result = persistence.insert(TABLE, user_to_entry(user)?).await?;

    Ok(result > 0)
}

pub async fn update_user(persistence: &Persistence, user: &User) -> Result<bool, AppError> {
    let result = persistence.update(TABLE, user_to_entry(user)?).await?;

    Ok(result > 0)
}

pub async fn delete_user(persistence: &Persistence, user_id: &str) -> Result<bool, AppError> {
    let result = persistence.delete(TABLE, user_id).await?;

    Ok(result > 0)
}

pub async fn load_all_users(persistence: &Persistence) -> Result<Vec<User>, AppError> {
    let user_entries = persistence.get_all(TABLE, Some("key")).await?;

    entries_to_users(user_entries)
}

pub async fn get_user(persistence: &Persistence, user_id: &str) -> Result<User, AppError> {
    match persistence.get(TABLE, user_id).await? {
        Some(entry) => entry_to_user(&entry),
        None => Err(AppError::UserNotFound(user_id.to_owned())),
    }
}
