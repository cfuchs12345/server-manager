mod conditions;

use crate::{commands, datastore, models::{plugin::sub_action::SubAction, server::{Server, Feature}, error::AppError, input::ActionOrDataInput}, models::response::data_result::ConditionCheckResult};


#[derive(PartialEq, Eq)]
pub enum CheckType {
    OnlyMainFeatures,
    OnlySubFeatures
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
    action_params: Option<&str>,
    crypto_key: String,
) -> Result<bool, AppError> {
    let plugin_res = datastore::get_plugin(feature.id.as_str());
    if plugin_res.is_none() {
        log::error!("Plugin not found {}", feature.id);
        return Ok(false);
    }

    let plugin = plugin_res.unwrap();

    match plugin.find_action(action_id) {
        Some(plugin_action) => {
            let input: ActionOrDataInput = ActionOrDataInput::get_input_from_action(
                plugin_action,
                action_params,
                &plugin,
                feature,
                crypto_key,
            );

            commands::execute_command(Some(server.ipaddress.clone()), &input)
                .await
                .map(|res| {
                    log::info!("Response for server action was: {:?}", res);
                    true
                })
        }
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, feature.id);
            log::error!("{}", error);
            Err(AppError::UnknownPluginAction(plugin.id, action_id.to_string()))
        }
    }
}



pub async fn check_action_conditions(server: &Server, sub_actions: Vec<SubAction>, crypto_key: &str) -> Vec<ConditionCheckResult>{
    let mut results = Vec::new();

    for sub_action in &sub_actions {
        if sub_action.feature_id.is_none() || sub_action.action_id.is_none() {
            continue;
        }
        let feature = server.find_feature(sub_action.feature_id.as_ref().unwrap().to_owned());
        let plugin = datastore::get_plugin(sub_action.feature_id.as_ref().unwrap().as_str());
        
        if plugin.is_none() {
            continue;
        }

        let res = conditions::check_condition_for_action_met(server, feature,  plugin.unwrap().find_action(sub_action.action_id.as_ref().unwrap().as_str()), sub_action.action_params.clone(), crypto_key).await;
        results.push(res);
    }
    results
}



pub async fn check_main_action_conditions() {
    let servers = datastore::get_all_servers();
    let crypto_key = datastore::get_crypto_key();

    let mut vec: Vec<ConditionCheckResult> = Vec::new();
    for server in servers {
        let mut res = conditions::check_all_action_conditions(server, &crypto_key, CheckType::OnlyMainFeatures).await;
        vec.append(&mut res);
    }
    
    for result in vec {
        datastore::insert_condition_result(result);
    }
}
