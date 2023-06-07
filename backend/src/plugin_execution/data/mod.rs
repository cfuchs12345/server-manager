mod conversion;

use async_recursion::async_recursion;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    commands::{self, http::HttpCommandResult, replace, socket::SocketCommandResult, CommandInput},
    datastore::{self},
    models::{
        error::AppError,
        plugin::{data::Data, sub_action::SubAction, Plugin},
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

            let data_responses = execute_specific_data_query(
                server,
                &plugin,
                feature,
                data,
                None,
                crypto_key.as_str(),
            )
            .await?;

            for data_response in data_responses {
                let inputs =
                    get_command_inputs(data, crypto_key.as_str(), None, feature, &plugin, server)
                        .await?;

                let enriched_result =
                    process_result_for_display(data, &data_response.1, template_engine, feature)?;

                for input in inputs {
                    let replaced = replace(enriched_result.as_str(), &input)?.1; // use second part of tuple - we don't want show passwords in the ouput if someone adds credentials placeholders in the template

                    let actions = extract_actions(&replaced);
                    let check_results = actions::check_action_conditions(
                        server.clone(),
                        actions,
                        crypto_key.clone(),
                    )
                    .await;

                    results.push(DataResult {
                        ipaddress: server.ipaddress,
                        result: replaced,
                        check_results,
                    });
                }
            }
        }
    }

    Ok(results)
}

fn process_result_for_display(
    data: &Data,
    response: &str,
    template_engine: &handlebars::Handlebars<'static>,
    feature: &Feature,
) -> Result<String, AppError> {
    let result = if !data.template.is_empty() {
        // convert the output with the template
        conversion::convert_result_string_to_html(
            data.template.as_str(),
            response.to_owned(),
            template_engine,
            data,
        )?
    } else {
        // no template - just append
        response.to_owned()
    };
    let enriched_result = inject_meta_data_for_actions(result, feature, data);
    Ok(enriched_result)
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
    action_params: Option<String>,
    crypto_key: &str,
) -> Result<Vec<(CommandInput, String)>, AppError> {
    let inputs =
        get_command_inputs(data, crypto_key, action_params, feature, plugin, server).await?;

    let mut responses = Vec::new();

    for input in inputs {
        let response = execute_command(input.clone()).await?;

        if let Some(script) = &data.post_process {
            log::trace!("before post process: {}", response);
            let response = super::pre_or_post_process(response.as_str(), script)?;
            log::trace!("after post process: {}", response);

            responses.push((input, response));
        } else {
            responses.push((input, response));
        }
    }

    Ok(responses)
}

async fn execute_command(input: CommandInput) -> Result<String, AppError> {
    let response = match input.get_name() {
        commands::socket::SOCKET => {
            let result: SocketCommandResult = commands::execute(input, false).await?;
            result.get_response()
        }
        _ => {
            let result: HttpCommandResult = commands::execute(input, false).await?;
            result.get_response()
        }
    };
    Ok(response)
}

#[async_recursion]
async fn get_command_inputs(
    data: &Data,
    crypto_key: &str,
    action_params: Option<String>,
    feature: &Feature,
    plugin: &Plugin,
    server: &Server,
) -> Result<Vec<CommandInput>, AppError> {
    let inputs = match data.command.as_str() {
        commands::socket::SOCKET => {
            commands::socket::make_command_input_from_data(
                server,
                crypto_key,
                data,
                action_params,
                feature,
                plugin,
            )
            .await?
        }
        _ => {
            commands::http::make_command_input_from_data(
                server,
                crypto_key,
                data,
                action_params,
                feature,
                plugin,
            )
            .await?
        }
    };
    Ok(inputs)
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
