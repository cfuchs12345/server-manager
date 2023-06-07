use futures::future::join_all;

use crate::{
    common, datastore,
    models::{
        error::AppError,
        plugin::{
            action::{Action, DependsDef, State},
            data::Data,
            Plugin,
        },
        server::{Feature, Server},
    },
    models::{response::data_result::ConditionCheckResult, response::status::Status},
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
    feature: Option<Feature>,
    action: Option<Action>,
    action_params: Option<String>,
    crypto_key: String,
) -> ConditionCheckResult {
    if feature.is_none() || action.is_none() {
        return ConditionCheckResult {
            ipaddress: server.ipaddress,
            result: false,
            action_id: "".to_string(),
            feature_id: "".to_string(),
            action_params: "".to_string(),
        };
    }

    let plugin_res = datastore::get_plugin(feature.as_ref().unwrap().id.as_str());

    if plugin_res.is_none() {
        return ConditionCheckResult {
            action_id: action.unwrap().id,
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id,
            ipaddress: server.ipaddress,
            result: false,
        };
    }

    let plugin = plugin_res.unwrap();

    let status: Vec<Status> =
        other_functions::statuscheck::status_check(vec![server.ipaddress], true)
            .await
            .unwrap_or(Vec::new());

    let mut result = match action.as_ref().unwrap().available_for_state {
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
        return ConditionCheckResult {
            action_id: action.unwrap().id,
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id,
            ipaddress: server.ipaddress,
            result: false,
        };
    }

    if let Some(status) = status.first() {
        if status.is_running {
            // if not running, no need to start any request
            // now check data dependencies one by one
            for depends in &action.as_ref().unwrap().depends {
                match find_data_for_action_condition(depends, &plugin) {
                    Some(data) => {
                        let responses = data::execute_specific_data_query(
                            &server,
                            &plugin,
                            feature.as_ref().unwrap(),
                            data,
                            action_params.clone(),
                            crypto_key.as_str(),
                        )
                        .await
                        .unwrap_or_default();

                        for response in &responses {
                            result &= response_data_match(depends, Some(response.1.clone()))
                                .unwrap_or_default();
                        }
                        if !result {
                            log::debug!("Dependencies for data {} of plugin {} for server {} not met. Responses were {:?}", data.id, feature.as_ref().unwrap().id, server.ipaddress, responses);
                            break;
                        }
                    }
                    None => {
                        let error = format!(
                            "dependent data with id  {} not found for action {}",
                            depends.data_id,
                            action.as_ref().unwrap().id
                        );
                        log::error!("{}", error);
                        result = false;
                        break;
                    }
                }
            }
        } else if !action.as_ref().unwrap().depends.is_empty() {
            result = false;
        }
    };

    ConditionCheckResult {
        action_id: action.as_ref().unwrap().id.clone(),
        action_params: action_params.unwrap_or_default(),
        feature_id: feature.unwrap().id,
        ipaddress: server.ipaddress,
        result,
    }
}

pub async fn check_all_action_conditions<'l>(
    server: Server,
    crypto_key: &str,
    check_type: CheckType,
) -> Vec<ConditionCheckResult> {
    let mut tasks = Vec::new();

    for feature in server.clone().features {
        let plugin_res = datastore::get_plugin(feature.id.as_str());
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

                tasks.push(tokio::spawn(async move {
                    check_condition_for_action_met(
                        server_clone,
                        Some(feature_clone),
                        Some(action_clone),
                        None,
                        crypto_key_clone,
                    )
                    .await
                }));
            }
        }
    }
    let result = join_all(tasks).await;
    log::debug!("Number of results is {}", result.len());

    let vec: Vec<ConditionCheckResult> = result
        .iter()
        .map(move |r| r.as_ref().unwrap().to_owned())
        .collect();

    vec
}

fn response_data_match(dependency: &DependsDef, input: Option<String>) -> Result<bool, AppError> {
    if input.is_none() {
        return Ok(false);
    }
    let script = dependency.script.clone();
    let script_type = dependency.script_type.clone();

    let is_lua = matches!(script_type.as_str(), "lua");
    let is_rhai = matches!(script_type.as_str(), "rhai");

    if is_lua {
        Ok(common::match_with_lua(input.unwrap().as_str(), &script))
    } else if is_rhai {
        Ok(common::match_with_rhai(input.unwrap().as_str(), &script))
    } else {
        Err(AppError::InvalidArgument(
            "script".to_string(),
            Some(script_type),
        ))
    }
}

fn find_data_for_action_condition<'a>(depend: &DependsDef, plugin: &'a Plugin) -> Option<&'a Data> {
    plugin.data.iter().find(|d| d.id == depend.data_id)
}
