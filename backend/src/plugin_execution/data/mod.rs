mod conversion;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    commands::{self, http::HttpCommandResult, socket::SocketCommandResult},
    common, datastore,
    models::{
        error::AppError,
        plugin::{common::Script, data::Data, sub_action::SubAction, Plugin},
        response::data_result::DataResult,
        server::{Feature, Server},
    },
    plugin_execution::actions,
};

lazy_static! {
    static ref TEMPLATE_SUB_ACTION_REGEX: Regex = Regex::new(r"(\[\[Action .*?\]\])").unwrap();
}

/// Executes all data queries on the given server for all given plugins
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `template_engine` - the template engine struct that is used to render the output in a readable format
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_data_query(
    server: &Server,
    template_engine: &handlebars::Handlebars<'static>,
    crypto_key: String,
) -> Result<Vec<DataResult>, AppError> {
    let mut results: Vec<DataResult> = vec![];

    for feature in &server.features {
        let plugin_opt = datastore::get_plugin(feature.id.as_str());

        if plugin_opt.is_none() {
            log::error!("plugin {} not found", feature.id);
            continue;
        }
        let plugin = plugin_opt.unwrap();

        for data in &plugin.data {
            log::debug!("Plugin data execute {} {}", plugin.id, data.id);

            if !data.output {
                log::debug!(
                    "Skipping data entry {} of plugin {} since it is marked as output = false",
                    data.id,
                    plugin.id
                );
                continue;
            }
            let data_response = execute_specific_data_query(
                server,
                &plugin,
                feature,
                data,
                None,
                crypto_key.as_str(),
            )
            .await?;

            if let Some(response) = data_response {
                let result = if !data.template.is_empty() {
                    // convert the output with the template
                    conversion::convert_result_string_to_html(
                        data.template.as_str(),
                        response,
                        template_engine,
                        data,
                    )?
                } else {
                    // no template - just append
                    response
                };

                let enriched_result = inject_meta_data_for_actions(result, feature, data);

                let actions = extract_actions(&enriched_result);

                let check_results =
                    actions::check_action_conditions(server.clone(), actions, crypto_key.clone())
                        .await;

                results.push(DataResult {
                    ipaddress: server.ipaddress,
                    result: enriched_result,
                    check_results,
                });
            }
        }
    }

    Ok(results)
}

/// Executes a specific data query on the given server for a given data point config of a plugin
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `plugin` - the plugin to which the data query belongs to
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `data` - the actual data query (as defined in the plugin) that should be executed
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_specific_data_query(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    data: &Data,
    action_params: Option<&str>,
    crypto_key: &str,
) -> Result<Option<String>, AppError> {
    let mut response = match data.command.as_str() {
        commands::socket::SOCKET => {
            let input = commands::socket::make_command_input_from_data(
                crypto_key,
                data,
                action_params,
                feature,
                plugin,
            )?;

            let result: SocketCommandResult = commands::execute(input, false).await?;
            result.get_response()
        }
        _ => {
            let input = commands::http::make_command_input_from_data(
                server,
                crypto_key,
                data,
                action_params,
                feature,
                plugin,
            )?;

            let result: HttpCommandResult = commands::execute(input, false).await?;
            result.get_response()
        }
    };

    if let Some(script) = &data.post_process {
        response = post_process(response.as_str(), script)?;
    }

    Ok(Some(response))
}

fn post_process(response: &str, script: &Script) -> Result<String, AppError> {
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

fn extract_actions(input: &str) -> Vec<SubAction> {
    let mut result = Vec::new();
    let groups: Vec<String> = TEMPLATE_SUB_ACTION_REGEX
        .find_iter(input)
        .map(|mat| mat.as_str().to_owned())
        .collect();
    for group in groups {
        result.push(SubAction::from(group));
    }

    result
}

fn inject_meta_data_for_actions(input: String, feature: &Feature, data: &Data) -> String {
    input.replace(
        "[[Action ",
        format!(
            "[[Action feature.id=\"{}\" data.id=\"{}\" ",
            feature.id, data.id
        )
        .as_str(),
    )
}
