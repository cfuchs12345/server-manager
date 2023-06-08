use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{common, datastore, other_functions};

pub async fn start_scheduled_jobs() {
    let scheduler = JobScheduler::new().await.unwrap();

    schedule_cache_update(&scheduler).await;
    schedule_status_check(&scheduler).await;
    schedule_condition_checks(&scheduler).await;
    schedule_token_cleanup(&scheduler).await;
    schedule_one_time_crypt_key_cleanup(&scheduler).await;
    schedule_monitoring(&scheduler).await;

    match scheduler.start().await {
        Ok(_res) => log::debug!("Schedulder started"),
        Err(err) => log::error!("Could not start schedulder due to : {}", err),
    }
}

async fn schedule_condition_checks(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new_async("1/10 * * * * *", |_uuid, _l| {
                Box::pin(async {
                    crate::plugin_execution::check_main_action_conditions(&true).await;
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

async fn schedule_status_check(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new_async("1/20 * * * * *", |_uuid, _l| {
                Box::pin(async {
                    other_functions::statuscheck::status_check_all(&true)
                        .await
                        .expect("Error during scheduled status check");
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

async fn schedule_monitoring(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new_async("1/20 * * * * *", |_uuid, _l| {
                Box::pin(async {
                    crate::plugin_execution::monitor_all(&true)
                        .await
                        .expect("Error during scheduled monitoring");
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

async fn schedule_cache_update(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new("1/30 * * * * *", |_uuid, _l| {
                datastore::update_cache();
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

async fn schedule_token_cleanup(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new("0 0 * * * *", |_uuid, _l| {
                crate::datastore::delete_expired_tokens();
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

async fn schedule_one_time_crypt_key_cleanup(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new("0 * * * * *", |_uuid, _l| {
                common::invalidate_expired_one_time_keys();
            })
            .unwrap(),
        )
        .await
        .unwrap();
}
