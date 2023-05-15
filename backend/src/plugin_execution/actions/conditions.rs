use rhai::{Scope, Engine};
use rlua::Lua;

use crate::{models::{response::data_result::ConditionCheckResult, response::status::Status}, plugin_execution::data, datastore, models::{plugin::{action::{State, Action, DependsDef}, Plugin, data::Data}, server::{Server, Feature}, error::AppError}, commands::ping};

use super::CheckType;


/// Checks if all conditions that are defined for an action of a plugin are met and that it can be executed by the user
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `actiond`- the plugin action to check
/// * `persistence` - the persistence struct that helps to interact with the underlying database

pub async fn check_condition_for_action_met(
    server: &Server,
    feature: Option<&Feature>,
    action: Option<&Action>,
    action_params: Option<String>,
    crypto_key: &str,
) -> ConditionCheckResult {
    if feature.is_none() || action.is_none()  {
        return ConditionCheckResult {
            ipaddress: server.ipaddress.clone(),
            result: false,
            ..Default::default()
        };
    }

    let plugin_res = datastore::get_plugin(feature.unwrap().id.as_str());

    if plugin_res.is_none()  {
        return ConditionCheckResult {
            action_id: action.unwrap().id.clone(),
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id.clone(),
            ipaddress: server.ipaddress.clone(),
            result: false,
        };
    }

    let plugin = plugin_res.unwrap();

    let status: Vec<Status> =
        ping::status_check(vec![server.ipaddress.clone()], true)
            .await
            .unwrap_or(Vec::new());

    let mut result = match action.unwrap().available_for_state {
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
            action_id: action.unwrap().id.clone(),
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id.clone(),
            ipaddress: server.ipaddress.clone(),
            result: false,
        };
    }

    if let Some(status) = status.first() {
        if status.is_running {
            // if not running, no need to start any request
            // now check data dependencies one by one
            for depends in &action.unwrap().depends {
                match find_data_for_action_condition(depends, &plugin) {
                    Some(data) => {
                        let response = data::execute_specific_data_query(
                            server,
                            &plugin,
                            feature.unwrap(),
                            data,
                            action_params.as_deref(),
                            crypto_key,
                        )
                        .await
                        .unwrap_or_default();

                        result &=
                            response_data_match(depends, response.clone()).unwrap_or_default();
                        if !result {
                            log::debug!("Depencies for data {} of plugin {} for server {} not met .Reasponse was {:?}", data.id, feature.unwrap().id, server.ipaddress, response);
                            break;
                        }
                    }
                    None => {
                        let error = format!(
                            "dependent data with id  {} not found for action {}",
                            depends.data_id, action.unwrap().id
                        );
                        log::error!("{}", error);
                        result = false;
                        break;
                    }
                }
            }
        } else if !action.unwrap().depends.is_empty() {
            result = false;
        }
    };

    ConditionCheckResult {
        action_id: action.unwrap().id.clone(),
        action_params: action_params.unwrap_or_default(),
        feature_id: feature.unwrap().id.clone(),
        ipaddress: server.ipaddress.clone(),
        result,
    }
}





pub async fn check_all_action_conditions(server: Server, crypto_key: &str, check_type: CheckType) -> Vec<ConditionCheckResult> {
    let mut vec = Vec::new();

    for feature in server.clone().features {
        let plugin_res = datastore::get_plugin(feature.id.as_str());
        if plugin_res.is_none() {
            log::error!("plugin with id {} not found", feature.id);
            continue;
        }
        if let Some(plugin) = plugin_res {
            for action in plugin.clone().actions {
                if check_type == CheckType::OnlyMainFeatures && !action.show_on_main {
                    log::debug!("Skipping action condition check for non-main action {} of feature {}", action.id, feature.id);
                    continue;
                }
                else if check_type == CheckType::OnlySubFeatures && action.show_on_main {
                    log::debug!("Skipping action condition check for sub-main action {} of feature {}", action.id, feature.id);
                    continue;
                }
                let server_clone = server.clone();
                let feature_clone = feature.clone();
                let action_clone = action.clone();

                let check_res = check_condition_for_action_met(
                    &server_clone,
                    Some(&feature_clone),
                    Some(&action_clone),
                    None,
                    crypto_key,
                )
                .await;

                vec.push(check_res);
            }
        }
    }

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

    if !is_lua && !is_rhai {
        return Err(AppError::InvalidArgument("script".to_string(), Some(script_type)));
    }

    let mut result = false;

    if is_lua {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
            globals
                .set("input", "[[".to_string() + input.unwrap().as_str() + "]]")
                .expect("Could not set global value");

            if let Ok(value) = lua_ctx.load(&script).eval() {
                result = value;
            }
        });
    } else if is_rhai {
        let mut scope = Scope::new();

        scope.push("input", input.unwrap());

        let engine = Engine::new();
        if let Ok(value) = engine.eval_with_scope::<bool>(&mut scope, &script) {
            result = value;
        }
    }

    Ok(result)
}



fn find_data_for_action_condition<'a>(depend: &DependsDef, plugin: &'a Plugin) -> Option<&'a Data> {
    plugin.data.iter().find(|d| d.id == depend.data_id)
}