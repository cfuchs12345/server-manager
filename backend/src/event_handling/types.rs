use std::{collections::HashMap, fmt::Debug};

use crate::models::error::AppError;

use super::ObjectType;

#[derive(PartialEq, Debug)]
pub enum Value {
    Text(String),
    Number(i64),
    Boolean(bool),
}

impl Value {
    pub fn different(&self, other: &Value) -> bool {
        self.get_value_as_astring() != other.get_value_as_astring()
    }

    fn get_value_as_astring(&self) -> String {
        match self {
            Value::Text(val) => val.to_owned(),
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
