use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::error::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    Insert,
    Update,
    Delete,
    Refresh,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectType {
    Status,
    Server,
    Plugin,
    DisabledPlugins,
    ConditionCheckResult,
    Notification,
    User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    occurrence_datetime: DateTime<Utc>,
    object_type: ObjectType,
    event_type: EventType,
    key_name: String,
    key: String,
    value: String,
    change_flag: String,
}

impl Event {
    pub fn new_from_event_source(
        occurrence_datetime: DateTime<Utc>,
        event_type: EventType,
        event_source: &dyn EventSource,
    ) -> Result<Self, AppError> {
        Ok(Event {
            occurrence_datetime,
            object_type: event_source.get_object_type(),
            event_type,
            key_name: event_source.get_event_key_name(),
            key: event_source.get_event_key(),
            value: event_source.get_event_value()?,
            change_flag: event_source.get_change_flag(),
        })
    }

    pub fn new_from_listevent_source(
        occurrence_datetime: DateTime<Utc>,
        event_type: EventType,
        event_source: &dyn EventSource,
        value: String,
    ) -> Result<Self, AppError> {
        Ok(Event {
            occurrence_datetime,
            object_type: event_source.get_object_type(),
            event_type,
            key_name: event_source.get_event_key_name(),
            key: event_source.get_event_key(),
            value,
            change_flag: event_source.get_change_flag(),
        })
    }
}

pub trait EventSource {
    fn get_object_type(&self) -> ObjectType;

    fn get_event_key_name(&self) -> String;

    fn get_event_key(&self) -> String;

    fn get_event_value(&self) -> Result<String, AppError>;

    fn get_change_flag(&self) -> String;
}

impl Debug for dyn EventSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EventSource{{{} {} {:?} {}}}",
            self.get_event_key_name(),
            self.get_event_key(),
            self.get_event_value(),
            self.get_change_flag()
        )
    }
}

pub struct ListSource {
    object_type: ObjectType,
    key_name: String,
    list: Vec<String>,
}

impl ListSource {
    pub fn new(object_type: ObjectType, list: Vec<String>) -> Self {
        ListSource {
            object_type,
            key_name: "id".to_owned(),
            list,
        }
    }

    pub fn diff(&self, date_time: DateTime<Utc>, other: Self) -> Result<Vec<Event>, AppError> {
        let mut res = Vec::new();

        for new_val in &self.list {
            if !other.list.contains(new_val) {
                let event = Event::new_from_listevent_source(
                    date_time,
                    EventType::Insert,
                    self,
                    new_val.to_owned(),
                )?;

                res.push(event);
            }
        }

        for old_val in other.list {
            if !self.list.contains(&old_val) {
                let event = Event::new_from_listevent_source(
                    date_time,
                    EventType::Delete,
                    self,
                    old_val.to_owned(),
                )?;

                res.push(event);
            }
        }
        Ok(res)
    }
}

impl EventSource for ListSource {
    fn get_object_type(&self) -> ObjectType {
        self.object_type.clone()
    }

    fn get_event_key_name(&self) -> String {
        self.key_name.to_string()
    }

    fn get_event_key(&self) -> String {
        "key".to_string()
    }

    fn get_event_value(&self) -> Result<String, AppError> {
        Ok("".to_string())
    }

    fn get_change_flag(&self) -> String {
        format!("{:?}", self.list)
    }
}
