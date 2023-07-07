use chrono::{DateTime, Utc};

use crate::models::error::AppError;

use super::types::{Event, EventSource, EventType};

pub fn get_event_for_refresh(
    occurrence_datetime: DateTime<Utc>,
    current: Box<dyn EventSource>,
) -> Result<Event, AppError> {
    log::debug!("refresh {:?} ", current);

    Event::new_from_event_source(occurrence_datetime, EventType::Refresh, &*current)
}

pub fn get_event_for_object_change(
    occurrence_datetime: DateTime<Utc>,
    current: Option<Box<dyn EventSource>>,
    old: Option<Box<dyn EventSource>>,
) -> Result<Option<Event>, AppError> {
    log::debug!("current {:?} old {:?}", current, old);

    let event = if current.is_none() && old.is_some() {
        Some(Event::new_from_event_source(
            occurrence_datetime,
            EventType::Delete,
            &*old.expect("could not get option value"),
        )?)
    } else if current.is_some() && old.is_none() {
        Some(Event::new_from_event_source(
            occurrence_datetime,
            EventType::Insert,
            &*current.expect("could not get option value"),
        )?)
    } else if current
        .as_ref()
        .expect("could not get option value")
        .get_change_flag()
        != old
            .as_ref()
            .expect("could not get option value")
            .get_change_flag()
    {
        Some(Event::new_from_event_source(
            occurrence_datetime,
            EventType::Update,
            &*current.expect("could not get option value"),
        )?)
    } else {
        log::debug!("Did not handle object change {:?} {:?}", current, old);
        None
    };

    log::trace!("{:?}", event);

    if let Some(event) = event {
        log::trace!("sending event {:?}", event);
        return Ok(Some(event));
    }

    Ok(None)
}
