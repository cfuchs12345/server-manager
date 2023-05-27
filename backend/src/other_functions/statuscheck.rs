use std::net::IpAddr;

use futures::future::join_all;
use lazy_static::lazy_static;
use tokio::sync::Semaphore;

use crate::{
    commands::{self, ping::PingCommandResult},
    datastore,
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

        tasks.push(tokio::spawn(commands::execute::<PingCommandResult>(input)));
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

    datastore::cache_status(results_from_query);

    drop(permit);
    Ok(())
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
