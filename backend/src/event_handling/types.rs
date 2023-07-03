use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::error::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    Insert,
    Update,
    Delete,
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
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    String(String),
    StringList(Vec<String>),
    Number(i64),
    Boolean(bool),
}

impl Value {
    pub fn different(&self, other: &Value) -> bool {
        self.get_value_as_astring() != other.get_value_as_astring()
    }

    fn get_value_as_astring(&self) -> String {
        match self {
            Value::String(val) => val.to_owned(),
            Value::StringList(val) => format!("{:?}", val),
            Value::Number(val) => format!("{}", val),
            Value::Boolean(val) => format!("{}", val),
        }
    }
}

pub trait EventSource {
    fn get_object_type(&self) -> ObjectType;

    fn get_event_key_name(&self) -> String;

    fn get_event_key(&self) -> String;

    fn get_event_value(&self) -> Result<String, AppError>;

    fn get_key_values(&self) -> HashMap<String, Value>;
}

impl Debug for dyn EventSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EventSource{{{} {} {:?}}}",
            self.get_event_key_name(),
            self.get_event_key(),
            self.get_event_value()
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

    fn get_key_values(&self) -> HashMap<String, Value> {
        let mut kv = HashMap::new();
        kv.insert("list".to_owned(), Value::StringList(self.list.clone()));
        kv
    }
}
