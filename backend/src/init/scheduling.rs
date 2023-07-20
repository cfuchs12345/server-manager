use std::collections::HashMap;
use std::time::Instant;

use chrono::Utc;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    common,
    datastore::{self},
    event_handling::{self, EventSource},
    models::{error::AppError, response::system_information::SystemInformation},
    other_functions::systeminfo,
};

lazy_static! {
    static ref LAST_RUN: RwLock<HashMap<String, Instant>> = RwLock::new(HashMap::new());
}

#[allow(dead_code)]
pub async fn get_last_run(name: &str) -> Option<Instant> {
    LAST_RUN.read().await.get(name).map(|v| v.to_owned())
}

pub async fn start_scheduled_jobs() -> Result<(), AppError> {
    let scheduler = JobScheduler::new().await?;

    //schedule_refresh(&scheduler).await?;
    schedule_heartbeat(&scheduler).await?;
    schedule_system_info_publish(&scheduler).await?;
    schedule_cache_update(&scheduler).await?;
    schedule_token_cleanup(&scheduler).await?;
    schedule_one_time_crypt_key_cleanup(&scheduler).await?;

    scheduler.start().await?;

    Ok(())
}

async fn schedule_heartbeat(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new("*/3 * * * * *", |_uuid, _l| {
            publish_heartbeat();
        })?)
        .await?;

    Ok(())
}

async fn schedule_system_info_publish(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new("*/20 * * * * *", |_uuid, _l| {
            let now = Utc::now();
            let system_info = SystemInformation {
                load_average: systeminfo::get_load_info(),
                memory_stats: systeminfo::get_memory_stats(),
                memory_usage: systeminfo::get_memory_usage(),
            };

            publish_system_info(now, system_info);
        })?)
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn schedule_refresh(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new("0 */10 * * * *", |_uuid, _l| {
            let now = Utc::now();
            match datastore::get_all_servers_from_cache() {
                Ok(servers) => {
                    for server in servers {
                        match datastore::get_status(&server.get_ipaddress()) {
                            Ok(status) => {
                                if let Some(status) = status {
                                    publish_refresh(now, Box::new(status));
                                };
                            }
                            Err(err) => {
                                log::error!(
                                    "Error while loading server status for server {:?}: {}",
                                    server,
                                    err
                                );
                            }
                        }
                        publish_refresh(now, Box::new(server));
                    }
                }
                Err(err) => {
                    log::error!("Error while loading servers for client refresh: {}", err);
                }
            }
        })?)
        .await?;

    Ok(())
}

fn publish_heartbeat() {
    match event_handling::publish_heartbeat() {
        Ok(_) => {
            log::debug!("published heartbeat");
        }
        Err(err) => {
            log::error!("Error while publishing heartbeat: {}", err);
        }
    }
}

fn publish_system_info(now: chrono::DateTime<Utc>, system_info: SystemInformation) {
    match event_handling::publish_system_info(now, &system_info) {
        Ok(_) => {
            log::debug!("published system information {:?}", system_info);
        }
        Err(err) => {
            log::error!("Error while publishing system info event: {}", err);
        }
    }
}

#[allow(dead_code)]
fn publish_refresh(now: chrono::DateTime<Utc>, object: Box<dyn EventSource>) {
    match event_handling::publish_refresh_event(now, object) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error while publishing refresh event: {}", err);
        }
    }
}

async fn schedule_cache_update(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new("1/30 * * * * *", |_uuid, _l| {
            datastore::update_cache();
        })?)
        .await?;

    Ok(())
}

async fn schedule_token_cleanup(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new(
            "0 0 * * * *",
            |_uuid, _l| match crate::datastore::delete_expired_tokens() {
                Ok(_) => {}
                Err(err) => {
                    log::error!(
                        "Could not execute job schedule_token_cleanup. Error was {}",
                        err
                    )
                }
            },
        )?)
        .await?;

    Ok(())
}

async fn schedule_one_time_crypt_key_cleanup(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new_async("0 * * * * *", |_uuid, _l| {
            Box::pin(async {
                match common::invalidate_expired_one_time_keys().await {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!(
                        "Could not execute job schedule_one_time_crypt_key_cleanup. Error was {}",
                        err
                    )
                    }
                }
            })
        })?)
        .await?;

    Ok(())
}
