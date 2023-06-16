use std::time::SystemTime;

use crate::common;

use self::response::status::Status;

pub mod config;
pub mod error;
pub mod plugin;
pub mod request;
pub mod response;
pub mod server;
pub mod timeseries;
pub mod token;
pub mod users;

pub use timeseries::{TimeSeriesData, TimeSeriesValue, Timestamp};

pub fn status_list_to_timeseries_data_list(status: Vec<Status>) -> Vec<TimeSeriesData> {
    let now = SystemTime::now();

    status
        .iter()
        .map(|s| TimeSeriesData {
            timestamp: Timestamp::SysTime(now),
            identifier: TimeSeriesValue::Symbol(
                common::IDENTIFIER.to_owned(),
                format!("{}", s.ipaddress),
            ),
            sub_identifiers: Vec::new(),
            value: TimeSeriesValue::Int(common::VALUE.to_owned(), bool_to_int(s.is_running)),
        })
        .collect()
}

fn bool_to_int(val: bool) -> i64 {
    match val {
        true => 1,
        false => 0,
    }
}
