use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::models::error::AppError;

use super::types::{Event, EventSource, EventType, Value};

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
    } else if current.is_some() && old.is_some() {
        let current_val = current
            .as_ref()
            .expect("could not get option value")
            .as_ref();
        let old_val = old.as_ref().expect("could not get option value").as_ref();

        if current_val.get_version() != old_val.get_version()
            || key_values_are_different(current_val.get_key_values(), old_val.get_key_values())
        {
            Some(Event::new_from_event_source(
                occurrence_datetime,
                EventType::Update,
                &*current.expect("could not get option value"),
            )?)
        } else {
            log::debug!("Did not handle object change {:?} {:?}", current, old);
            None
        }
    } else {
        log::warn!("both values seem to be empty: {:?} {:?}", current, old);
        None
    };

    log::trace!("{:?}", event);

    if let Some(event) = event {
        log::trace!("sending event {:?}", event);
        return Ok(Some(event));
    }

    Ok(None)
}

fn key_values_are_different(
    current_map: HashMap<String, Value>,
    old_map: HashMap<String, Value>,
) -> bool {
    for (k, v) in &current_map {
        let old = old_map.get(k);

        if old.is_none()
            || v != old.expect("could not get option value")
            || v.different(old.expect("could not get option value"))
        {
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
