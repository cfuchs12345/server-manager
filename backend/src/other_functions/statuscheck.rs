use std::net::IpAddr;

use futures::future::join_all;
use lazy_static::lazy_static;
use tokio::sync::Semaphore;

use crate::{
    commands::{self, ping::PingCommandResult},
    datastore::{self, TimeSeriesPersistence},
    models::{self, error::AppError, response::status::Status},
};

lazy_static! {
    static ref SEMAPHORE_STATUS_CHECK: Semaphore = Semaphore::new(1);
}

pub async fn status_check_all(silent: &bool) -> Result<(), AppError> {
    let servers = datastore::get_all_servers_from_cache()?;
    let permit = SEMAPHORE_STATUS_CHECK.acquire().await?;
    // list of async tasks executed by tokio
    let mut tasks = Vec::new();

    for server in servers {
        let address: IpAddr = server.get_ipaddress();

        let input = commands::ping::make_input(address);
        let silent = *silent;

        tasks.push(tokio::spawn(async move {
            let ip = input.get_ipaddress();

            match commands::execute::<PingCommandResult>(input, &silent).await {
                Ok(res) => {
                    let status = Status::from(res);

                    match datastore::cache_status(&[status.clone()]) {
                        Ok(_) => {}
                        Err(err) => log::error!("Could not chache status. Error was: {}", err),
                    }
                    status
                }
                Err(err) => {
                    log::error!("Error during statuc check: {}", err);
                    Status::error(ip.unwrap())
                }
            }
        }));
    }

    // wait for all tasks to finish
    let task_results = join_all(tasks).await;

    let results_from_query: Vec<Status> = task_results
        .iter()
        .map(move |res| res.as_ref().expect("Could not get ref").to_owned())
        .collect();

    log::debug!("inserting {} status into cache", results_from_query.len());

    let data = models::status_list_to_timeseries_data_list(results_from_query);

    datastore::save_timeseries_data(
        &mut TimeSeriesPersistence::new().await?,
        "server_status",
        data,
    )
    .await?;

    drop(permit);
    Ok(())
}

pub async fn status_check(
    ips_to_check: Vec<IpAddr>,
    use_cache: bool,
) -> Result<Vec<Status>, AppError> {
    let list_to_check = if ips_to_check.is_empty() {
        datastore::get_all_servers_from_cache()?
            .iter()
            .map(|s| s.get_ipaddress())
            .collect()
    } else {
        ips_to_check
    };

    let result = if use_cache {
        list_to_check
            .iter()
            .map(|ipaddress| {
                datastore::get_status(ipaddress)
                    .unwrap_or(Some(Status::new(ipaddress.to_owned())))
                    .unwrap_or_else(|| Status::new(ipaddress.to_owned()))
            })
            .collect()
    } else {
        Vec::new()
    };
    Ok(result)
}
