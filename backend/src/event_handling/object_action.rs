use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::error::AppError;

use super::types::{Event, EventSource, EventType, ListSource, Value};

pub fn get_events_for_list_change(
    occurrence_datetime: DateTime<Utc>,
    current: ListSource,
    old: ListSource,
) -> Result<Vec<Event>, AppError> {
    let current_map = current.get_key_values();
    let old_map = old.get_key_values();

    for v in current_map.values() {
        for v_old in old_map.values() {
            if let Value::StringList(list) = v {
                if let Value::StringList(list_old) = v_old {
                    return diff_lists(list, list_old, occurrence_datetime, &current);
                }
            }
        }
    }

    Ok(Vec::new())
}

fn diff_lists(
    list: &Vec<String>,
    list_old: &Vec<String>,
    now: DateTime<Utc>,
    current: &ListSource,
) -> Result<Vec<Event>, AppError> {
    let mut res = Vec::new();

    for new_val in list {
        if !list_old.contains(new_val) {
            let event = Event::new_from_listevent_source(
                now,
                EventType::Insert,
                current,
                new_val.to_owned(),
            )?;

            res.push(event);
        }
    }

    for old_val in list_old {
        if !list.contains(old_val) {
            let event = Event::new_from_listevent_source(
                now,
                EventType::Delete,
                current,
                old_val.to_owned(),
            )?;

            res.push(event);
        }
    }

    Ok(res)
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
    } else if key_values_are_different(
        current
            .as_ref()
            .expect("could not get option value")
            .get_key_values(),
        old.as_ref()
            .expect("could not get option value")
            .get_key_values(),
    ) {
        Some(Event::new_from_event_source(
            occurrence_datetime,
            EventType::Update,
            &*current.expect("could not get option value"),
        )?)
    } else {
        None
    };

    log::debug!("{:?}", event);

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
