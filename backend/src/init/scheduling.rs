use std::sync::atomic::AtomicBool;
use std::time::Instant;
use std::{collections::HashMap, time::Duration};

use lazy_static::lazy_static;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    common,
    datastore::{self},
    models::error::AppError,
    other_functions,
};

const MONITORING_INTERVAL: u64 = 5;
const NOTIFICATION_INTERVAL: u64 = 30;

lazy_static! {
    static ref LAST_RUN: RwLock<HashMap<String, Instant>> = RwLock::new(HashMap::new());
}

pub async fn get_last_run(name: &str) -> Option<Instant> {
    LAST_RUN.read().await.get(name).map(|v| v.to_owned())
}

pub async fn start_scheduled_jobs() -> Result<(), AppError> {
    let scheduler = JobScheduler::new().await?;

    schedule_cache_update(&scheduler).await?;
    schedule_status_check(&scheduler).await?;
    schedule_condition_checks(&scheduler).await?;
    schedule_token_cleanup(&scheduler).await?;
    schedule_one_time_crypt_key_cleanup(&scheduler).await?;
    schedule_monitoring(&scheduler).await?;

    scheduler.start().await?;

    Ok(())
}

async fn schedule_condition_checks(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new_async("1/20 * * * * *", |_uuid, _l| {
            Box::pin(async {
                log::info!("condition_checks");

                match crate::plugin_execution::check_main_action_conditions(&true).await {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!(
                            "Could not execute job schedule_condition_checks. Error was {}",
                            err
                        )
                    }
                }
            })
        })?)
        .await?;

    Ok(())
}

async fn schedule_status_check(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new_async("1/20 * * * * *", |_uuid, _l| {
            Box::pin(async {
                log::info!("status_check");

                other_functions::statuscheck::status_check_all(&true)
                    .await
                    .expect("Error during scheduled status check");
            })
        })?)
        .await?;

    Ok(())
}

async fn schedule_monitoring(scheduler: &JobScheduler) -> Result<(), AppError> {
    scheduler
        .add(Job::new_async("1/20 * * * * *", |_uuid, _l| {
            Box::pin(async {
                log::info!("status_monitoring");

                let intervals = get_intervals();

                match crate::plugin_execution::execute_all_data_dependent(
                    &true,
                    get_last_run("schedule_monitoring").await,
                    Duration::from_secs(intervals.0),
                    Duration::from_secs(intervals.1),
                )
                .await
                {
                    Ok(_) => {
                        LAST_RUN
                            .write()
                            .await
                            .insert("schedule_monitoring".to_owned(), Instant::now());
                    }
                    Err(err) => {
                        log::error!("error while executing schedule_monitoring: {}", err);
                    }
                }
            })
        })?)
        .await?;

    Ok(())
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

fn get_intervals() -> (u64, u64) {
    datastore::get_config()
        .map(|c| {
            (
                c.get_int("monitoring_interval")
                    .map(|v| v as u64)
                    .unwrap_or(MONITORING_INTERVAL),
                c.get_int("notification_interval")
                    .map(|v| v as u64)
                    .unwrap_or(NOTIFICATION_INTERVAL),
            )
        })
        .unwrap_or((MONITORING_INTERVAL, NOTIFICATION_INTERVAL))
}
