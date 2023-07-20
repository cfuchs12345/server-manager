use std::any::Any;
use std::net::IpAddr;

use async_trait::async_trait;

use super::{common, Command, CommandInput, CommandResult, Parameters};
use crate::commands::common::replace;
use crate::models::error::AppError;
use crate::models::plugin::action::ActionDef;
use crate::models::plugin::data::DataDef;
use crate::models::plugin::detection::DetectionEntry;
use crate::models::plugin::Plugin;
use crate::models::server::{Feature, Server};

pub const KAFKA: &str = "kafka";

#[derive(Clone)]
pub struct KafkaCommand {}
impl KafkaCommand {
    pub fn new() -> Self {
        KafkaCommand {}
    }
}

#[async_trait]
impl Command for KafkaCommand {
    fn get_name(&self) -> &str {
        KAFKA
    }

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError> {
        let command = input.find_single_arg("command")?;
        let topic = input.find_param("topic")?;
        let response_topic = input.find_param("response_topic")?;
        let timeout = input
            .find_single_arg("timeout")
            .unwrap_or(input.find_param("timeout").unwrap_or("5"));

        let normal_and_masked_command: (String, String) = replace::replace(command, input)?;

        log::debug!("{}", normal_and_masked_command.0);

        let response_string = crate::common::execute_kafka_request(
            input.get_ipaddress(),
            topic,
            response_topic,
            normal_and_masked_command.0.as_str(),
            chrono::Duration::seconds(timeout.parse()?),
        )
        .await?;

        Ok(Box::new(KafkaCommandResult::new(response_string.as_str())))
    }
}

#[derive(Clone)]
pub struct KafkaCommandResult {
    response: String,
}
impl KafkaCommandResult {
    fn new(response: &str) -> Self {
        KafkaCommandResult {
            response: response.to_owned(),
        }
    }

    pub fn get_response(&self) -> String {
        self.response.clone()
    }
}

impl CommandResult for KafkaCommandResult {}

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
            KAFKA,
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
            KAFKA,
            Some(crypto_key),
            Some(server.get_ipaddress()),
            args_list,
            params.clone(),
            feature.credentials.clone(),
        ));
    }
    Ok(vec)
}

#[allow(dead_code)]
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
            KAFKA,
            None,
            Some(ipaddress.to_owned()),
            args_list,
            params.clone(),
            Vec::new(),
        ));
    }
    Ok(vec)
}
