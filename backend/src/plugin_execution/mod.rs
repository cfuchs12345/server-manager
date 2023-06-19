mod actions;
mod data;
mod discovery;
mod monitoring;
mod notifications;

use std::time::{Duration, Instant};

pub use discovery::auto_discover_servers_in_network;
pub use discovery::discover_features;
pub use discovery::discover_features_of_all_servers;

pub use actions::check_main_action_conditions;
pub use actions::execute_action;

pub use data::execute_data_query;
pub use data::execute_specific_data_query;

pub use monitoring::get_monitoring_data;

use crate::common;
use crate::datastore;
use crate::datastore::Persistence;
use crate::models::error::AppError;
use crate::models::plugin::common::Script;
use crate::models::plugin::Plugin;
use crate::plugin_execution::monitoring::MonitoringProcessor;
use crate::plugin_execution::notifications::NotificationProcessor;

pub fn pre_or_post_process(response: &str, script: &Script) -> Result<String, AppError> {
    common::script_process(script, response)
}

pub async fn execute_all_data_dependent(
    persistence: &Persistence,
    silent: &bool,
    last_run: Option<Instant>,
    monitoring_interval: Duration,
    notification_interval: Duration,
) -> Result<(), AppError> {
    let servers = datastore::get_all_servers_from_cache()?;
    let plugins = datastore::get_all_plugins()?;
    let crypto_key = datastore::get_crypto_key()?;

    let relevant_plugins: Vec<Plugin> = plugins
        .iter()
        .filter(|p| p.data.iter().any(|d| !d.monitoring.is_empty()))
        .map(|p| p.to_owned())
        .collect();

    let mut monitoring_processor = MonitoringProcessor::new(last_run, monitoring_interval);
    let mut notification_processor =
        NotificationProcessor::new(last_run, notification_interval, persistence);

    log::trace!("relevant plugins for monitoring: {:?}", &relevant_plugins);

    for server in servers {
        for plugin in &relevant_plugins {
            let feature = server.find_feature(plugin.id.as_str());

            if let Some(feature) = feature {
                log::trace!("Server {:?} has relevant feature {:?}", server, feature);

                for data in &plugin.data {
                    if !monitoring_processor.is_relevant_data_for_processing(data)
                        && !notification_processor.is_relevant_data_for_processing(data, plugin)
                    {
                        continue;
                    }

                    match data::execute_specific_data_query(
                        &server,
                        plugin,
                        &feature,
                        data,
                        None,
                        crypto_key.as_str(),
                        silent, // silent - no error log
                    )
                    .await
                    {
                        Ok(input_response_tuples) => {
                            // let both start in parallel
                            let monit_process =
                                monitoring_processor.process(data, &input_response_tuples);

                            let notify_process = notification_processor.process(
                                plugin,
                                data,
                                &input_response_tuples,
                            );

                            monit_process.await?;
                            notify_process.await?;
                        }
                        Err(err) => match err {
                            AppError::Suppressed(err) => {
                                log::debug!("suppressed error: {}", err);
                            }
                            y => {
                                return Err(y);
                            }
                        },
                    }
                }
            } else {
                log::debug!(
                        "Feature {:?} not relevant for Server {:?} it only has following features: {:?}",
                        feature,
                        server,
                        server.features
                    );
            }
        }
    }

    // let both start in parallel
    let not_finish = notification_processor.finish();
    let mon_finish = monitoring_processor.finish();

    not_finish.await?;
    mon_finish.await?;

    Ok(())
}
