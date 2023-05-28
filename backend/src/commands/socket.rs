use std::{any::Any, net::IpAddr};

use super::{
    common::{self, replace},
    Command, CommandInput, CommandResult, Parameters,
};
use crate::models::{
    error::AppError,
    plugin::{action::Action, data::Data, detection::DetectionEntry, Plugin},
    server::{Feature, Server},
};
use async_trait::async_trait;

pub const SOCKET: &str = "socket";

#[derive(Clone)]
pub struct SocketCommand {}
impl SocketCommand {
    pub fn new() -> Self {
        SocketCommand {}
    }
}

#[async_trait]
impl Command for SocketCommand {
    fn get_name(&self) -> &str {
        SOCKET
    }

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError> {
        let url = input.find_single_arg("url")?;
        let method = input.find_single_arg("method")?;
        let headers = input.find_all_args("header")?;
        let socket = input.find_single_arg("socket")?;

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

        #[cfg(all(target_os = "linux"))]
        let response_string = execute_windows_dummy(
            socket,
            normal_and_masked_url.0.as_str(),
            method,
            Some(normal_and_replaced_headers),
            Some(normal_and_masked_body.0),
        );

        #[cfg(all(target_os = "windows"))]
        let response_string = execute_windows_dummy(
            socket,
            normal_and_masked_url.0.as_str(),
            method,
            Some(normal_and_replaced_headers),
            Some(normal_and_masked_body.0),
        );

        Ok(Box::new(SocketCommandResult::new(response_string.as_str())))
    }
}

#[cfg(all(target_os = "linux"))]
fn execute_linux(
    socket: &str,
    url: &str,
    method: &str,
    request_headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> Response<String, AppError> {
    crate::common::execute_socket_request(
        socket,
        normal_and_masked_url.0.as_str(),
        method,
        Some(normal_and_replaced_headers),
        Some(normal_and_masked_body.0),
    )
    .await?
}

#[cfg(all(target_os = "windows"))]
fn execute_windows_dummy(
    _socket: &str,
    _url: &str,
    _method: &str,
    _request_headers: Option<Vec<(String, String)>>,
    _body: Option<String>,
) -> String {
    "".to_string()
}

#[derive(Clone)]
pub struct SocketCommandResult {
    response: String,
}
impl SocketCommandResult {
    fn new(response: &str) -> Self {
        SocketCommandResult {
            response: response.to_owned(),
        }
    }

    pub fn get_response(&self) -> String {
        self.response.clone()
    }
}

impl CommandResult for SocketCommandResult {}

pub fn make_command_input_from_subaction(
    server: &Server,
    crypto_key: &str,
    action: &Action,
    action_params: Option<&str>,
    feature: &Feature,
    plugin: &Plugin,
) -> Result<CommandInput, AppError> {
    let params = Parameters::new(
        common::string_params_to_command_args(action_params),
        common::params_to_command_args(&feature.params),
        common::param_def_to_command_args(&plugin.params),
    );

    Ok(CommandInput::new(
        SOCKET,
        Some(crypto_key),
        Some(server.ipaddress),
        common::args_to_command_args(&action.args),
        params,
        feature.credentials.clone(),
    ))
}

#[allow(dead_code)]
pub fn make_command_input_from_data(
    server: &Server,
    crypto_key: &str,
    data: &Data,
    action_params: Option<&str>,
    feature: &Feature,
    plugin: &Plugin,
) -> Result<CommandInput, AppError> {
    let params = Parameters::new(
        common::string_params_to_command_args(action_params),
        common::params_to_command_args(&feature.params),
        common::param_def_to_command_args(&plugin.params),
    );

    Ok(CommandInput::new(
        SOCKET,
        Some(crypto_key),
        Some(server.ipaddress),
        common::args_to_command_args(&data.args),
        params,
        feature.credentials.clone(),
    ))
}

pub fn make_command_input_from_detection(
    ipaddress: &IpAddr,
    detection_entry: &DetectionEntry,
) -> Result<CommandInput, AppError> {
    let params = Parameters::new(
        Vec::new(),
        Vec::new(),
        common::param_def_to_command_args(&detection_entry.params),
    );

    Ok(CommandInput::new(
        SOCKET,
        None,
        Some(ipaddress.to_owned()),
        common::args_to_command_args(&detection_entry.args),
        params,
        Vec::new(),
    ))
}
