use tokio_cron_scheduler::{Job, JobScheduler};


use crate::{datastore, commands::ping};


pub async fn start_scheduled_jobs() {
    let scheduler = JobScheduler::new().await.unwrap();

    schedule_cache_update(&scheduler).await;
    schedule_status_check(&scheduler).await;
    schedule_condition_checks(&scheduler).await;

    match scheduler.start().await {
        Ok(_res) => log::info!("Schedulder started"),
        Err(err) => log::error!("Could not start schedulder due to : {}", err),
    }
}

async fn schedule_condition_checks(scheduler: &JobScheduler) {
    scheduler.add(
        Job::new_async("1/10 * * * * *", |_uuid, _l| {
            Box::pin(async {
                    crate::plugin_execution::check_main_action_conditions().await;
            })
        })
        .unwrap(),

    ) .await.unwrap();
}

async fn schedule_status_check(scheduler: &JobScheduler) {
    scheduler
        .add(
            Job::new_async("1/5 * * * * *", |_uuid, _l| {
                Box::pin(async {
                    ping::status_check_all().await;
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
            Job::new("1/10 * * * * *", |_uuid, _l| {
                datastore::update_cache();
            })
            .unwrap(),
        )
        .await
        .unwrap();
}
