use std::{collections::HashMap, net::IpAddr};

use crate::{
    common, datastore,
    models::{
        error::AppError,
        plugin::{
            action::{ActionDef, DependsDef, State},
            data::DataDef,
            Plugin,
        },
        server::{Feature, Server},
    },
    models::{
        response::data_result::ConditionCheckResult,
        response::{data_result::ConditionCheckSubResult, status::Status},
    },
    other_functions,
    plugin_execution::data,
};

use super::CheckType;

/// Checks if all conditions that are defined for an action of a plugin are met and that it can be executed by the user
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `actiond`- the plugin action to check
/// * `persistence` - the persistence struct that helps to interact with the underlying database

pub async fn check_condition_for_action_met(
    server: Server,
    data_id: String,
    feature: Option<Feature>,
    action: Option<ActionDef>,
    action_params: Option<String>,
    crypto_key: String,
    silent: &bool,
) -> Result<ConditionCheckResult, AppError> {
    if feature.is_none() || action.is_none() {
        return Ok(ConditionCheckResult {
            ipaddress: server.ipaddress,
            data_id,
            subresults: vec![ConditionCheckSubResult {
                result: false,
                action_id: "".to_string(),
                feature_id: "".to_string(),
                action_params: "".to_string(),
            }],
        });
    }

    let Some(plugin) = datastore::get_plugin(feature.as_ref().expect("Could not get ref").id.as_str())? else {
        return Ok(ConditionCheckResult {
            ipaddress: server.ipaddress,            
            data_id,
            subresults: vec![ConditionCheckSubResult {
                action_id: action.expect("checked before").id,
                action_params: action_params.unwrap_or_default(),
                feature_id: feature.expect("checked before").id,
                result: false,
            }],
        });
    };

    let status: Vec<Status> =
        other_functions::statuscheck::status_check(vec![server.ipaddress], true)
            .await
            .unwrap_or(Vec::new());

    let mut result = match action
        .as_ref()
        .expect("Could not get ref")
        .available_for_state
    {
        State::Active => {
            match status.first() {
                Some(status) => status.is_running,
                None => false, // unknown state - better do not allow an action
            }
        }
        State::Inactive => {
            match status.first() {
                Some(status) => !status.is_running,
                None => false, // unknown state - better do not allow an action
            }
        }
        State::Any => {
            // no status check - just allow it
            true
        }
    };

    if !result {
        // check if status dependency already failed - early exit
        return Ok(ConditionCheckResult {
            ipaddress: server.ipaddress,
            data_id,
            subresults: vec![ConditionCheckSubResult {
                action_id: action.expect("checked before").id,
                action_params: action_params.unwrap_or_default(),
                feature_id: feature.expect("checked before").id,
                result: false,
            }],
        });
    }

    if let Some(status) = status.first() {
        if status.is_running {
            // if not running, no need to start any request
            // now check data dependencies one by one
            for depends in &action.as_ref().expect("Could not get ref").depends {
                match find_data_for_action_condition(depends, &plugin) {
                    Some(data) => {
                        let responses = data::execute_specific_data_query(
                            &server,
                            &plugin,
                            feature.as_ref().expect("Could not get ref"),
                            data,
                            action_params.clone(),
                            crypto_key.as_str(),
                            silent, // silent check - no error log
                        )
                        .await
                        .unwrap_or_default();

                        for response in &responses {
                            result &= response_data_match(depends, Some(response.1.clone()))
                                .unwrap_or_default();
                        }
                        if !result {
                            log::debug!("Dependencies for data {} of plugin {} for server {} not met. Responses were {:?}", data.id, feature.as_ref().expect("Could not get ref").id, server.ipaddress, responses);
                            break;
                        }
                    }
                    None => {
                        let error = format!(
                            "dependent data with id  {} not found for action {}",
                            depends.data_id,
                            action.as_ref().expect("Could not get ref").id
                        );
                        log::error!("{}", error);
                        result = false;
                        break;
                    }
                }
            }
        } else if !action
            .as_ref()
            .expect("Could not get ref")
            .depends
            .is_empty()
        {
            result = false;
        }
    };

    Ok(ConditionCheckResult {
        ipaddress: server.ipaddress,
        data_id,
        subresults: vec![ConditionCheckSubResult {
            action_id: action.as_ref().expect("Could not get ref").id.clone(),
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.expect("checked before").id,
            result,
        }],
    })
}

pub async fn check_all_action_conditions<'l>(
    server: Server,
    crypto_key: &str,
    check_type: CheckType,
    silent: &bool,
) -> Result<(), AppError> {
    let mut vec = Vec::new();

    for feature in server.clone().features {
        let plugin_res = datastore::get_plugin(feature.id.as_str())?;
        if plugin_res.is_none() {
            log::error!("plugin with id {} not found", feature.id);
            continue;
        }
        if let Some(plugin) = plugin_res {
            for action in plugin.clone().actions {
                if check_type == CheckType::OnlyMainFeatures && !action.show_on_main {
                    log::debug!(
                        "Skipping action condition check for non-main action {} of feature {}",
                        action.id,
                        feature.id
                    );
                    continue;
                } else if check_type == CheckType::OnlySubFeatures && action.show_on_main {
                    log::debug!(
                        "Skipping action condition check for sub-main action {} of feature {}",
                        action.id,
                        feature.id
                    );
                    continue;
                }
                let server_clone = server.clone();
                let feature_clone = feature.clone();
                let action_clone = action.clone();
                let crypto_key_clone = crypto_key.to_owned().clone();

                let silent = silent.to_owned();

                let cr = check_condition_for_action_met(
                    server_clone,
                    "".to_owned(),
                    Some(feature_clone),
                    Some(action_clone),
                    None,
                    crypto_key_clone,
                    &silent,
                )
                .await?;

                vec.push(cr);
            }
        }
    }

    log::debug!("Number of results is {}", vec.len());
    let merged = super::merge_condition_check_results(vec);
    log::debug!("Number of results after merge is {}", merged.len());

    for cr in merged {
        datastore::insert_condition_result(cr)?;
    }
    Ok(())
}

fn response_data_match(dependency: &DependsDef, input: Option<String>) -> Result<bool, AppError> {
    match input {
        Some(input) => common::script_match(&dependency.script, input.as_str()),
        None => Ok(false),
    }
}

fn find_data_for_action_condition<'a>(
    depend: &DependsDef,
    plugin: &'a Plugin,
) -> Option<&'a DataDef> {
    plugin.data.iter().find(|d| d.id == depend.data_id)
}
