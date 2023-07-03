use std::{collections::HashMap, thread, time::Duration};

use chrono::prelude::*;

use futures_util::Future;
use lazy_static::lazy_static;
use tokio::{runtime::Runtime, sync::RwLock, time::sleep};

use crate::models::error::AppError;

lazy_static! {
    static ref EXECUTIONS: RwLock<ProcessExecutions> = RwLock::new(ProcessExecutions {
        executions: HashMap::new()
    });
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum ProcessType {
    Monitoring,
    StatusCheck,
    ConditionCheck,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct ProcessExecution {
    process_type: ProcessType,
    prev_start: Option<DateTime<Utc>>,
    prev_end: Option<DateTime<Utc>>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

impl ProcessExecution {
    pub fn new(process_type: ProcessType, start: DateTime<Utc>) -> Self {
        ProcessExecution {
            process_type,
            prev_start: None,
            prev_end: None,
            start: Some(start),
            end: None,
        }
    }

    pub fn set_end(&mut self, end: DateTime<Utc>) {
        self.end = Some(end);

        log::info!(
            "{}: started {:?} at {} ended at {}. Took {} seconds",
            thread::current().name().unwrap_or_default(),
            self.process_type,
            self.start.unwrap_or_default(),
            self.end.unwrap_or_default(),
            self.time_taken()
        );
    }

    pub fn set_start(&mut self, start: DateTime<Utc>) {
        self.copy_current_to_prev();
        self.start = Some(start);
        self.end = None;

        log::info!(
            "{}: started {:?} at {}",
            thread::current().name().unwrap_or_default(),
            self.process_type,
            start
        );
    }

    fn copy_current_to_prev(&mut self) {
        self.prev_start = self.start;
        self.prev_end = self.end;
    }

    fn time_taken(&self) -> i64 {
        let Some(s) = self.start else {
            return 0;
        };
        let Some(e) = self.end else {
            return 0;
        };
        e.signed_duration_since(s).num_seconds()
    }
}

#[derive(Debug)]
struct ProcessExecutions {
    executions: HashMap<ProcessType, ProcessExecution>,
}

impl ProcessExecutions {
    pub fn set_start(&mut self, process_type: &ProcessType) {
        if let Some(existing) = self.executions.get_mut(process_type) {
            existing.set_start(Utc::now())
        } else {
            self.executions.insert(
                process_type.clone(),
                ProcessExecution::new(process_type.clone(), Utc::now()),
            );
        }
    }
    pub fn set_end(&mut self, process_type: &ProcessType) {
        if let Some(existing) = self.executions.get_mut(process_type) {
            existing.set_end(Utc::now())
        } else {
            log::error!("Should log process end, but found no entry with the start value. This should not happen!?");
        }
    }

    pub fn time_taken(&self, process_type: &ProcessType) -> u64 {
        let Some(existing) = self.executions.get(process_type) else {
            return 0;
        };
        existing.time_taken() as u64
    }
}

pub async fn start_background_prcesses() -> Result<(), AppError> {
    println!("start_background_prcesses");
    start_condition_checks().await;
    start_status_checks().await;
    start_monitoring().await;

    Ok(())
}

async fn start_condition_checks() {
    thread::spawn(|| {
        futures::executor::block_on(async {
            let rt = Runtime::new().unwrap();
            // Spawn a future onto the runtime

            rt.spawn(async {
                loop {
                    run(ProcessType::ConditionCheck, condition_check).await;
                }
            })
            .await
        })
    });
}

async fn start_status_checks() {
    thread::spawn(|| {
        futures::executor::block_on(async {
            let rt = Runtime::new().unwrap();
            // Spawn a future onto the runtime

            rt.spawn(async {
                loop {
                    run(ProcessType::StatusCheck, status_check).await;
                }
            })
            .await
        })
    });
}

async fn start_monitoring() {
    thread::spawn(|| {
        futures::executor::block_on(async {
            let rt = Runtime::new().unwrap();
            // Spawn a future onto the runtime

            rt.spawn(async {
                loop {
                    run(ProcessType::Monitoring, monitoring).await;
                }
            })
            .await
        })
    });
}

async fn condition_check() {
    match crate::plugin_execution::check_main_action_conditions(true).await {
        Ok(_) => {}
        Err(err) => {
            log::error!(
                "Could not execute process condition_check. Error was {}",
                err
            )
        }
    }
}

async fn status_check() {
    match crate::other_functions::statuscheck::status_check_all(&true).await {
        Ok(_) => {}
        Err(err) => {
            log::error!("Could not execute process status_check. Error was {}", err)
        }
    }
}

async fn monitoring() {
    match crate::plugin_execution::execute_all_data_dependent(&true).await {
        Ok(_) => {}
        Err(err) => {
            log::error!("Could not execute process monitoring. Error was {}", err)
        }
    }
}

async fn run<F, Fut>(process_type: ProcessType, function: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    pre_function_exec(&process_type).await;

    function().await;

    post_function_exec(&process_type).await;
}

async fn post_function_exec(process_type: &ProcessType) {
    let mut lock = EXECUTIONS.write().await;
    let time_taken = lock.time_taken(process_type);
    lock.set_end(process_type);
    drop(lock);

    if time_taken < 5 {
        sleep(Duration::from_secs(5 - time_taken)).await; // if processes are really fast, we delay them up to 5 seconds
    } else {
        sleep(Duration::from_secs(2)).await;
    }
}

async fn pre_function_exec(process_type: &ProcessType) {
    let mut lock = EXECUTIONS.write().await;
    lock.set_start(process_type);
    drop(lock);
}
