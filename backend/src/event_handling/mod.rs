use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::models::error::AppError;

use self::types::EventSource;
use self::types::Value;

pub mod types;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    Insert,
    Update,
    Delete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectType {
    Status,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    object_type: ObjectType,
    event_type: EventType,
    key_name: String,
    key: String,
    value: String,
}

impl Event {
    pub fn new(event_type: EventType, event_source: &dyn EventSource) -> Result<Self, AppError> {
        Ok(Event {
            object_type: event_source.get_object_type(),
            event_type,
            key_name: event_source.get_event_key_name(),
            key: event_source.get_event_key(),
            value: event_source.get_event_value()?,
        })
    }
}

const MESSAGE_BUFFER_SIZE: usize = 50;

lazy_static! {
    static ref BUS: Mutex<(Sender<Event>, Receiver<Event>)> =
        Mutex::new(broadcast::channel(MESSAGE_BUFFER_SIZE));
}

pub async fn subscribe() -> Receiver<Event> {
    BUS.lock().unwrap().0.subscribe()
}

pub fn publish(event: Event) -> Result<usize, AppError> {
    BUS.lock()
        .unwrap()
        .0
        .send(event)
        .map_err(|err| AppError::CannotBroadcastEvent(format!("{:?}", err)))
}

pub fn handle_object_action(
    current: Option<Box<dyn EventSource>>,
    old: Option<Box<dyn EventSource>>,
) -> Result<(), AppError> {
    log::trace!("current {:?} old {:?}", current, old);

    let event = if current.is_none() && old.is_some() {
        Some(Event::new(EventType::Delete, &*old.unwrap())?)
    } else if current.is_some() && old.is_none() {
        Some(Event::new(EventType::Insert, &*current.unwrap())?)
    } else if key_values_are_different(
        current.as_ref().unwrap().get_key_values(),
        old.as_ref().unwrap().get_key_values(),
    ) {
        Some(Event::new(EventType::Update, &*current.unwrap())?)
    } else {
        None
    };

    if let Some(event) = event {
        log::trace!("sending event {:?}", event);
        publish(event)?;
    }

    Ok(())
}

fn key_values_are_different(
    current_map: HashMap<String, Value>,
    old_map: HashMap<String, Value>,
) -> bool {
    for (k, v) in &current_map {
        let old = old_map.get(k);

        if old.is_none() || v != old.unwrap() || v.different(old.unwrap()) {
            return true;
        }
    }

    for (k, _) in old_map {
        let current = current_map.get(&k);

        if current.is_none() {
            return true;
        }
    }
    false
}
