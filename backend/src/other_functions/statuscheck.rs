use std::{net::IpAddr, time::SystemTime};

use futures::future::join_all;
use lazy_static::lazy_static;
use tokio::sync::Semaphore;

use crate::{
    commands::{self, ping::PingCommandResult},
    common,
    datastore::{self, TimeSeriesData, TimeSeriesPersistence, TimeSeriesValue, Timestamp},
    models::{error::AppError, response::status::Status},
};

lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);
}

pub async fn status_check_all() -> Result<(), AppError> {
    let servers = datastore::get_all_servers();

    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await.unwrap();

    // list of async tasks executed by tokio
    let mut tasks = Vec::new();

    for server in servers {
        let address: IpAddr = server.ipaddress;

        let input = commands::ping::make_input(address);

        tasks.push(tokio::spawn(commands::execute::<PingCommandResult>(
            input, true,
        )));
    }

    // wait for all tasks to finish
    let task_results = join_all(tasks).await;

    let results_from_query: Vec<Status> = task_results
        .iter()
        .map(move |res| res.as_ref().unwrap().to_owned())
        .flat_map(|res| res.as_ref().ok())
        .map(|ping_result| Status::from(ping_result.to_owned()))
        .collect();

    log::debug!("inserting {} status into cache", results_from_query.len());

    datastore::cache_status(&results_from_query);

    let data = time_series_data_collection(results_from_query);

    datastore::save_timeseries_data(
        &mut TimeSeriesPersistence::new().await,
        "server_status",
        data,
    )
    .await?;

    drop(permit);
    Ok(())
}

fn time_series_data_collection(status: Vec<Status>) -> Vec<TimeSeriesData> {
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

pub async fn status_check(
    ips_to_check: Vec<IpAddr>,
    use_cache: bool,
) -> Result<Vec<Status>, AppError> {
    let list_to_check = if ips_to_check.is_empty() {
        datastore::get_all_servers()
            .iter()
            .map(|s| s.ipaddress)
            .collect()
    } else {
        ips_to_check
    };

    let result = if use_cache {
        list_to_check
            .iter()
            .map(|ipaddress| {
                datastore::get_status(ipaddress)
                    .unwrap_or_else(|| Status::new(ipaddress.to_owned()))
            })
            .collect()
    } else {
        Vec::new()
    };
    Ok(result)
}
