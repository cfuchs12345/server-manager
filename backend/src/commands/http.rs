use std::any::Any;

use super::{
    common::{self, replace},
    Command, CommandInput, CommandResult, Parameters,
};
use crate::models::{
    error::AppError,
    plugin::{action::Action, data::Data, Plugin},
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
        log::info!(
            "{} {} {}",
            normal_and_masked_url.0,
            method,
            normal_and_masked_body.0
        );

        let response_string = crate::common::execute_http_request(
            normal_and_masked_url.0,
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

pub fn make_command_input_from_subaction(
    server: &Server,
    crypto_key: &str,
    action: &Action,
    action_params: Option<&str>,
    feature: &Feature,
    plugin: &Plugin,
) -> Result<CommandInput, AppError> {
    let params = Parameters::new(
        common::action_params_to_command_args(action_params),
        common::feature_params_to_command_args(feature),
        common::plugin_default_params_to_command_args(plugin),
    );

    Ok(CommandInput::new(
        HTTP,
        Some(crypto_key),
        Some(server.ipaddress),
        common::action_args_to_command_args(action),
        params,
        feature.credentials.clone(),
    ))
}

pub fn make_command_input_from_data(
    server: &Server,
    crypto_key: &str,
    data: &Data,
    action_params: Option<&str>,
    feature: &Feature,
    plugin: &Plugin,
) -> Result<CommandInput, AppError> {
    let params = Parameters::new(
        common::action_params_to_command_args(action_params),
        common::feature_params_to_command_args(feature),
        common::plugin_default_params_to_command_args(plugin),
    );

    Ok(CommandInput::new(
        HTTP,
        Some(crypto_key),
        Some(server.ipaddress),
        common::data_args_to_command_args(data),
        params,
        feature.credentials.clone(),
    ))
}
