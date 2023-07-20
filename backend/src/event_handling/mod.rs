use std::sync::Mutex;

use chrono::DateTime;
use chrono::Utc;
use lazy_static::lazy_static;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::models::error::AppError;
use crate::models::response::system_information::SystemInformation;

pub use self::types::Event;

pub use types::{EventSource, ListSource, ObjectType, Value};

mod object_action;
mod types;

const MESSAGE_BUFFER_SIZE: usize = 500;

lazy_static! {
    static ref BUS: Mutex<(Sender<Event>, Receiver<Event>)> =
        Mutex::new(broadcast::channel(MESSAGE_BUFFER_SIZE));
}

pub fn publish_heartbeat() -> Result<(), AppError> {
    publish(Event::new_heartbeat()?)?;
    Ok(())
}

pub fn publish_system_info(
    occurrence_datetime: DateTime<Utc>,
    system_info: &SystemInformation,
) -> Result<(), AppError> {
    publish(Event::new_from_system_info(
        occurrence_datetime,
        system_info,
    )?)?;
    Ok(())
}

#[allow(dead_code)]
pub fn publish_refresh_event(
    occurrence_datetime: DateTime<Utc>,
    current: Box<dyn EventSource>,
) -> Result<(), AppError> {
    let event = object_action::get_event_for_refresh(occurrence_datetime, current)?;

    publish(event)?;
    Ok(())
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
    for event in current.diff(now, old)? {
        publish(event)?;
    }

    Ok(())
}

pub async fn subscribe() -> Result<Receiver<Event>, AppError> {
    Ok(BUS
        .lock()
        .map_err(|err| AppError::CannotSubscriveToEvents(format!("{:?}", err)))?
        .0
        .subscribe())
}

fn publish(event: Event) -> Result<usize, AppError> {
    BUS.lock()
        .map_err(|err| AppError::CannotBroadcastEvent(format!("{:?}", err)))?
        .0
        .send(event)
        .map_err(|err| AppError::CannotBroadcastEvent(format!("{:?}", err)))
}
