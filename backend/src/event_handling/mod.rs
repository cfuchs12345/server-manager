use std::sync::Mutex;

use chrono::Utc;
use lazy_static::lazy_static;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::models::error::AppError;

use self::types::Event;

pub use types::{EventSource, ListSource, ObjectType, Value};

mod object_action;
mod types;

const MESSAGE_BUFFER_SIZE: usize = 50;

lazy_static! {
    static ref BUS: Mutex<(Sender<Event>, Receiver<Event>)> =
        Mutex::new(broadcast::channel(MESSAGE_BUFFER_SIZE));
}

pub fn handle_object_change(
    current: Option<Box<dyn EventSource>>,
    old: Option<Box<dyn EventSource>>,
) -> Result<(), AppError> {
    let now = Utc::now();

    if let Some(event) = object_action::get_event_for_object_change(now, current, old)? {
        publish(event)?;
    }
    Ok(())
}

pub fn handle_list_change(current: ListSource, old: ListSource) -> Result<(), AppError> {
    let now = Utc::now();

    let events = object_action::get_events_for_list_change(now, current, old)?;
    for event in events {
        publish(event)?;
    }

    Ok(())
}

pub async fn subscribe() -> Receiver<Event> {
    BUS.lock().expect("could not lock bus").0.subscribe()
}

fn publish(event: Event) -> Result<usize, AppError> {
    BUS.lock()
        .expect("Could not publish event since bus could not be locked")
        .0
        .send(event)
        .map_err(|err| AppError::CannotBroadcastEvent(format!("{:?}", err)))
}
