mod actions;
mod data;
mod discovery;
mod monitoring;

pub use discovery::auto_discover_servers_in_network;
pub use discovery::discover_features;
pub use discovery::discover_features_of_all_servers;

pub use actions::check_main_action_conditions;
pub use actions::execute_action;

pub use data::execute_data_query;
pub use monitoring::monitor_all;

pub use monitoring::get_monitoring_data;

use crate::common;
use crate::models::error::AppError;
use crate::models::plugin::common::Script;

pub fn pre_or_post_process(response: &str, script: &Script) -> Result<String, AppError> {
    let is_lua = matches!(script.script_type.as_str(), "lua");
    let is_rhai = matches!(script.script_type.as_str(), "rhai");

    if is_lua {
        common::process_with_lua(response, &script.script)
    } else if is_rhai {
        common::process_with_rhai(response, &script.script)
    } else {
        Err(AppError::InvalidArgument(
            "script".to_string(),
            Some(script.script_type.clone()),
        ))
    }
}
