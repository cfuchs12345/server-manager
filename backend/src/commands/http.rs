use std::{any::Any, net::IpAddr};

use super::{
    common::{self, replace},
    Command, CommandInput, CommandResult, Parameters,
};
use crate::models::{
    error::AppError,
    plugin::{action::ActionDef, data::DataDef, detection::DetectionEntry, Plugin},
    server::{Feature, Server},
};
use async_trait::async_trait;

pub const HTTP: &str = "http";

#[derive(Clone)]
pub struct HttpCommand {}
impl HttpCommand {
    pub fn new() -> Self {
        HttpCommand {}
    }
}

#[async_trait]
impl Command for HttpCommand {
    fn get_name(&self) -> &str {
        HTTP
    }

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError> {
        let url = input.find_single_arg("url")?;
        let method = input.find_single_arg("method")?;
        let headers = input.find_all_args("header")?;

        let body: &str = match method {
            "post" => input.find_single_arg("body").unwrap_or({
                log::warn!(
                    "Actually expected a body for a post request. Continuing with an empty body."
                );
                ""
            }),
            "put" => input.find_single_arg("body").unwrap_or({
                log::error!(
                    "Actually expected a body for a put request. Continuing with an empty body."
                );
                ""
            }),
            _ => "",
        };

        let normal_and_masked_url: (String, String) = replace::replace(url, input)?;
        let normal_and_masked_body: (String, String) = replace::replace(body, input)?;
        let normal_and_replaced_headers: Vec<(String, String)> =
            replace::replace_list(headers, input)?;

        if !body.is_empty() {
            log::debug!(
                "About to execute method {} on url {} with body {}",
                method,
                normal_and_masked_url.1,
                normal_and_masked_body.1
            );

            log::debug!(
                "About to execute method {} on url {} with body {}",
                method,
                normal_and_masked_url.0,
                normal_and_masked_body.0
            );
        } else {
            log::debug!(
                "About to execute method {} on url {}",
                method,
                normal_and_masked_url.1
            );

            log::debug!(
                "About to execute method {} on url {}",
                method,
                normal_and_masked_url.0
            );
        }

        if normal_and_masked_url.0.trim().is_empty() {
            log::warn!(
            "Given url is empty after replacing placeholders. Was before replace: {}. Request will not be executed",
            url
        );
            return Err(AppError::InvalidArgument("url".to_string(), None));
        }
        log::debug!(
            "{} {} {}",
            normal_and_masked_url.0,
            method,
            normal_and_masked_body.0
        );

        let response_string = crate::common::execute_http_request(
            normal_and_masked_url.0.as_str(),
            method,
            Some(normal_and_replaced_headers),
            Some(normal_and_masked_body.0),
        )
        .await?;

        Ok(Box::new(HttpCommandResult::new(response_string.as_str())))
    }
}

#[derive(Clone)]
pub struct HttpCommandResult {
    response: String,
}
impl HttpCommandResult {
    fn new(response: &str) -> Self {
        HttpCommandResult {
            response: response.to_owned(),
        }
    }

    pub fn get_response(&self) -> String {
        self.response.clone()
    }
}

impl CommandResult for HttpCommandResult {}

pub async fn make_command_input_from_subaction(
    server: &Server,
    crypto_key: &str,
    action: &ActionDef,
    action_params: Option<String>,
    feature: &Feature,
    plugin: &Plugin,
    silent: &bool,
) -> Result<Vec<CommandInput>, AppError> {
    let params = Parameters::new(
        common::string_params_to_command_args(action_params)?,
        common::params_to_command_args(&feature.params),
        common::param_def_to_command_args(&plugin.params),
    );

    let mut vec = Vec::new();

    let list_of_args_list =
        common::args_to_command_args(&action.args, server, plugin, crypto_key, silent).await?;

    for args_list in list_of_args_list {
        vec.push(CommandInput::new(
            HTTP,
            Some(crypto_key),
            Some(server.get_ipaddress()),
            args_list,
            params.clone(),
            feature.credentials.clone(),
        ));
    }
    Ok(vec)
}

pub async fn make_command_input_from_data(
    server: &Server,
    crypto_key: &str,
    data: &DataDef,
    action_params: Option<String>,
    feature: &Feature,
    plugin: &Plugin,
    silent: &bool,
) -> Result<Vec<CommandInput>, AppError> {
    let params = Parameters::new(
        common::string_params_to_command_args(action_params)?,
        common::params_to_command_args(&feature.params),
        common::param_def_to_command_args(&plugin.params),
    );

    let mut vec = Vec::new();

    let list_of_args_list =
        common::args_to_command_args(&data.args, server, plugin, crypto_key, silent).await?;

    for args_list in list_of_args_list {
        vec.push(CommandInput::new(
            HTTP,
            Some(crypto_key),
            Some(server.get_ipaddress()),
            args_list,
            params.clone(),
            feature.credentials.clone(),
        ));
    }
    Ok(vec)
}

pub async fn make_command_input_from_detection(
    ipaddress: &IpAddr,
    crypto_key: &str,
    plugin: &Plugin,
    detection_entry: &DetectionEntry,
    silent: &bool,
) -> Result<Vec<CommandInput>, AppError> {
    let params = Parameters::new(
        Vec::new(),
        Vec::new(),
        common::param_def_to_command_args(&detection_entry.params),
    );

    let mut vec = Vec::new();

    let server = Server::new_only_ip(*ipaddress);

    let list_of_args_list =
        common::args_to_command_args(&detection_entry.args, &server, plugin, crypto_key, silent)
            .await?;

    for args_list in list_of_args_list {
        vec.push(CommandInput::new(
            HTTP,
            None,
            Some(ipaddress.to_owned()),
            args_list,
            params.clone(),
            Vec::new(),
        ));
    }
    Ok(vec)
}
