mod conditions;

use futures::future::join_all;

use crate::{
    commands::{
        self, http::HttpCommandResult, ping::PingCommandResult, socket::SocketCommandResult,
        wol::WolCommandResult,
    },
    datastore,
    models::response::data_result::ConditionCheckResult,
    models::{
        error::AppError,
        plugin::sub_action::SubAction,
        server::{Feature, Server},
    },
};

#[derive(PartialEq, Eq)]
pub enum CheckType {
    OnlyMainFeatures,
    OnlySubFeatures,
}

/// Executes a defined action of a plugin on the given server
/// # Arguments
///
/// * `server` - the server struct representing the server on which the action should be executed
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `action_id`- the identifier of the action
/// * `action_params`- optional params for an action
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_action(
    server: &Server,
    feature: &Feature,
    action_id: &str,
    action_params: Option<String>,
    crypto_key: String,
    silent: &bool,
) -> Result<bool, AppError> {
    let plugin = datastore::get_plugin(feature.id.as_str())?
        .ok_or(AppError::UnknownPlugin(feature.id.clone()))?;

    match plugin.find_action(action_id) {
        Some(plugin_action) => match plugin_action.command.as_str() {
            commands::http::HTTP => {
                let inputs = commands::http::make_command_input_from_subaction(
                    server,
                    &crypto_key,
                    plugin_action,
                    action_params,
                    feature,
                    &plugin,
                    silent,
                )
                .await?;

                let mut results: Vec<HttpCommandResult> = Vec::new();
                for input in inputs {
                    results.push(commands::execute(input, silent).await?);
                }

                Ok(results.iter().any(|r| !r.get_response().is_empty()))
            }
            commands::socket::SOCKET => {
                let inputs = commands::socket::make_command_input_from_subaction(
                    server,
                    &crypto_key,
                    plugin_action,
                    action_params,
                    feature,
                    &plugin,
                    silent,
                )
                .await?;
                let mut results: Vec<SocketCommandResult> = Vec::new();
                for input in inputs {
                    results.push(commands::execute(input, silent).await?);
                }

                Ok(results.iter().any(|r| !r.get_response().is_empty()))
            }
            commands::wol::WOL => {
                let input = commands::wol::make_input(feature);

                let res: WolCommandResult = commands::execute(input, silent).await?;

                Ok(res.get_result())
            }
            commands::ping::PING => {
                let input = commands::ping::make_input(server.ipaddress);

                let res: PingCommandResult = commands::execute(input, silent).await?;

                Ok(res.get_result())
            }
            y => {
                log::error!("Unknown command {}", y);
                Err(AppError::CommandNotFound(y.to_string()))
            }
        },
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, feature.id);
            log::error!("{}", error);
            Err(AppError::UnknownPluginAction(
                plugin.id,
                action_id.to_string(),
            ))
        }
    }
}

pub async fn check_action_conditions(
    server: Server,
    sub_actions: Vec<SubAction>,
    crypto_key: String,
    silent: &bool,
) -> Result<Vec<ConditionCheckResult>, AppError> {
    let mut tasks = Vec::new();
    for sub_action in &sub_actions {
        if sub_action.feature_id.is_none() || sub_action.action_id.is_none() {
            continue;
        }

        let feature =
            server.find_feature(sub_action.feature_id.as_ref().expect("Could not get ref"));
        let plugin = datastore::get_plugin(
            sub_action
                .feature_id
                .as_ref()
                .expect("Could not get ref")
                .as_str(),
        )?;

        if plugin.is_none() {
            continue;
        }
        let silent = silent.to_owned();
        let crypto_key = crypto_key.clone();
        let server = server.clone();
        let sub_action = sub_action.clone();

        tasks.push(tokio::spawn(async move {
            conditions::check_condition_for_action_met(
                server.clone(),
                feature,
                plugin
                    .expect("should not happen - checked before")
                    .find_action(
                        sub_action
                            .action_id
                            .as_ref()
                            .expect("Could not get ref")
                            .as_str(),
                    )
                    .map(|v| v.to_owned()),
                sub_action.action_params.clone(),
                crypto_key.clone(),
                &silent,
            )
            .await
        }));
    }
    let results = join_all(tasks).await;

    let vec: Vec<ConditionCheckResult> = results
        .iter()
        .map(|v| v.as_ref().expect("Could not get ref"))
        .map(|r| r.as_ref().expect("Could not get ref").to_owned())
        .collect();
    log::debug!("number of results is {}", vec.len());
    Ok(vec)
}

pub async fn check_main_action_conditions(silent: &bool) -> Result<(), AppError> {
    let servers = datastore::get_all_servers_from_cache()?;
    let crypto_key = datastore::get_crypto_key()?;

    let mut vec: Vec<ConditionCheckResult> = Vec::new();
    for server in servers {
        let mut res = conditions::check_all_action_conditions(
            server,
            &crypto_key,
            CheckType::OnlyMainFeatures,
            silent,
        )
        .await?;
        vec.append(&mut res);
    }

    for result in vec {
        datastore::insert_condition_result(result)?;
    }
    Ok(())
}
