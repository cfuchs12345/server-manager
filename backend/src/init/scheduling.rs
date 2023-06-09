use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{common, datastore, models::error::AppError, other_functions};

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
        .add(Job::new_async("1/10 * * * * *", |_uuid, _l| {
            Box::pin(async {
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
                crate::plugin_execution::monitor_all(&true)
                    .await
                    .expect("Error during scheduled monitoring");
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
