use std::{collections::HashMap, vec};

use crate::models::{error::AppError, plugin::notification::Notification};

use super::{Entry, Persistence};

const TABLE: &str = "notifications";

fn entry_to_notifications(entry: &Entry) -> Result<Vec<Notification>, AppError> {
    serde_json::from_str::<Vec<Notification>>(entry.value.as_str()).map_err(AppError::from)
}

fn entries_to_notifications(
    entries: Vec<Entry>,
) -> Result<HashMap<String, Vec<Notification>>, AppError> {
    let mut map = HashMap::new();

    for entry in &entries {
        let notifications = entry_to_notifications(entry)?;

        for notification in &notifications {
            map.entry(notification.ipaddress.clone())
                .or_insert_with(Vec::new)
                .push(notification.clone());
        }
    }
    Ok(map)
}

fn notifications_to_entry(notifications: &[Notification]) -> Result<Option<Entry>, AppError> {
    if notifications.is_empty() {
        return Ok(None);
    }

    Ok(Some(Entry {
        key: notifications
            .first()
            .map(|n| n.ipaddress.clone())
            .ok_or(AppError::Unknown(
                "expected at least one notification to save".to_owned(),
            ))?,
        value: serde_json::to_string(notifications)?,
    }))
}

pub async fn insert_or_update_notifications(
    persistence: &Persistence,
    notifications: &[Notification],
) -> Result<bool, AppError> {
    let mut updated = false;

    for notification in notifications {
        let existing: Vec<Notification> = get_existing(persistence, notification).await?;

        let vec = vec![notification.to_owned()];

        if existing.is_empty() {
            updated |= insert_notifications(persistence, vec.as_slice()).await?;
        } else {
            updated |= update_notifications(persistence, vec.as_slice()).await?;
        }
    }
    Ok(updated)
}

async fn get_existing(
    persistence: &Persistence,
    notification: &Notification,
) -> Result<Vec<Notification>, AppError> {
    let existing = match get_notification(persistence, &notification.ipaddress).await {
        Ok(existing) => existing,
        Err(err) => match err {
            AppError::DataNotFound(_) => Vec::new(),
            y => {
                log::error!("error while trying to select data: {}", y);
                return Err(y);
            }
        },
    };
    Ok(existing)
}

pub async fn insert_notifications(
    persistence: &Persistence,
    notifications: &[Notification],
) -> Result<bool, AppError> {
    if let Some(entry) = notifications_to_entry(notifications)? {
        let count = persistence.insert(TABLE, entry).await?;
        Ok(count > 0)
    } else {
        Ok(true)
    }
}

pub async fn update_notifications(
    persistence: &Persistence,
    notifications: &[Notification],
) -> Result<bool, AppError> {
    if let Some(entry) = notifications_to_entry(notifications)? {
        let result = persistence.update(TABLE, entry).await?;

        Ok(result > 0)
    } else {
        Ok(true)
    }
}

pub async fn delete_notification(
    persistence: &Persistence,
    ipaddress: &str,
) -> Result<bool, AppError> {
    let result = persistence.delete(TABLE, ipaddress).await?;

    Ok(result > 0)
}

pub async fn get_all_notifications(
    persistence: &Persistence,
) -> Result<HashMap<String, Vec<Notification>>, AppError> {
    let notification_entries = persistence.get_all(TABLE, Some("key")).await?;

    entries_to_notifications(notification_entries)
}

pub async fn get_notification(
    persistence: &Persistence,
    ipaddress: &str,
) -> Result<Vec<Notification>, AppError> {
    match persistence.get(TABLE, ipaddress).await? {
        Some(entry) => entry_to_notifications(&entry),
        None => Err(AppError::DataNotFound(ipaddress.to_owned())),
    }
}
