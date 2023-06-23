use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use serde::Serialize;

use crate::{common, send_response};

const PING: &str = "ping";

static HANDLE: Mutex<Option<ScheduleHandle>> = Mutex::new(None);

#[derive(Debug, Serialize)]
struct Heartbeat {
    message: String,
    timestamp: DateTime<Utc>,
}

impl Heartbeat {
    fn new() -> Self {
        Heartbeat {
            message: PING.to_owned(),
            timestamp: Utc::now(),
        }
    }
}

pub fn start_heartbeat(heartbeat_topic: &str, brokers: &[String]) {
    let mut scheduler = Scheduler::new();
    let topic = Arc::new(heartbeat_topic.to_owned());
    let brokers = Arc::new(brokers.to_owned());

    scheduler
        .every(2.seconds())
        .run(move || heartbeat(topic.clone(), brokers.clone()));

    *HANDLE.lock().unwrap() = Some(scheduler.watch_thread(common::FIVE_SECS));
}

fn heartbeat(topic: Arc<String>, brokers: Arc<Vec<String>>) {
    let hb = Heartbeat::new();
    let msg = serde_json::to_string(&hb).unwrap();

    send_response(msg.as_str(), topic.as_str(), &brokers);
}
