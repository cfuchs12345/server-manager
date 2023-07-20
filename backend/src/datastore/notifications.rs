use crate::models::{error::AppError, plugin::notification::Notifications};

use super::{persistence, Entry};

const TABLE: &str = "notifications";

fn entry_to_notifications(entry: &Entry) -> Result<Notifications, AppError> {
    serde_json::from_str::<Notifications>(entry.value.as_str()).map_err(AppError::from)
}

fn entries_to_notifications(entries: Vec<Entry>) -> Result<Vec<Notifications>, AppError> {
    let mut vec = Vec::new();

    for entry in &entries {
        let notifications = entry_to_notifications(entry)?;
        vec.push(notifications);
    }
    Ok(vec)
}

fn notifications_to_entry(notifications: Notifications) -> Result<Option<Entry>, AppError> {
    Ok(Some(Entry {
        key: notifications.ipaddress.clone(),
        value: serde_json::to_string(&notifications)?,
    }))
}

pub async fn insert_or_update_notifications(
    notifications: Notifications,
) -> Result<bool, AppError> {
    let mut updated = false;

    let existing = get_existing(&notifications).await?;

    if existing.is_none() {
        updated |= insert_notifications(notifications).await?;
    } else {
        updated |= update_notifications(notifications).await?;
    }
    Ok(updated)
}

async fn get_existing(notifications: &Notifications) -> Result<Option<Notifications>, AppError> {
    let existing = match get_notification(&notifications.ipaddress).await {
        Ok(existing) => Some(existing),
        Err(err) => match err {
            AppError::DataNotFound(_) => None,
            y => {
                log::error!("error while trying to select data: {}", y);
                return Err(y);
            }
        },
    };
    Ok(existing)
}

pub async fn insert_notifications(notifications: Notifications) -> Result<bool, AppError> {
    if let Some(entry) = notifications_to_entry(notifications)? {
        let count = persistence::insert(TABLE, entry).await?;
        Ok(count > 0)
    } else {
        Ok(true)
    }
}

pub async fn update_notifications(notifications: Notifications) -> Result<bool, AppError> {
    if let Some(entry) = notifications_to_entry(notifications)? {
        let result = persistence::update(TABLE, entry).await?;

        Ok(result > 0)
    } else {
        Ok(true)
    }
}

#[allow(dead_code)]
pub async fn delete_notification(ipaddress: &str) -> Result<bool, AppError> {
    let result = persistence::delete(TABLE, ipaddress).await?;

    Ok(result > 0)
}

pub async fn get_all_notifications() -> Result<Vec<Notifications>, AppError> {
    let notification_entries = persistence::get_all(TABLE, Some("key")).await?;

    entries_to_notifications(notification_entries)
}

pub async fn get_notification(ipaddress: &str) -> Result<Notifications, AppError> {
    match persistence::get(TABLE, ipaddress).await? {
        Some(entry) => entry_to_notifications(&entry),
        None => Err(AppError::DataNotFound(ipaddress.to_owned())),
    }
}
