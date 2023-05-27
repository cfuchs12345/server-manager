use std::{any::Any, net::IpAddr};

use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::models::{error::AppError, server::Credential};

mod common;
pub mod http;
pub mod ping;
pub mod socket;
pub mod wol;

lazy_static! {
    static ref COMMANDS: RwLock<Vec<Box<dyn Command + Sync + Send>>> =
        RwLock::new(get_command_list());
}

fn get_command_list() -> Vec<Box<dyn Command + Sync + Send>> {
    let list: Vec<Box<dyn Command + Sync + Send>> = vec![
        Box::new(http::HttpCommand::new()),
        Box::new(wol::WoLCommand::new()),
        Box::new(ping::PingCommand::new()),
    ];
    list
}

pub async fn execute<'a, R>(input: CommandInput) -> Result<R, AppError>
where
    R: CommandResult + Clone + 'a + 'static,
{
    let commands = COMMANDS.try_read().unwrap();

    match commands.iter().find(|c| c.can_handle(&input)) {
        Some(cmd) => match cmd.execute(&input).await {
            Ok(res) => {
                let res = res.downcast::<R>().map_err(|_err| {
                    AppError::Unknown("Could not cast result to specific type".to_string())
                })?;
                Ok(res.as_ref().clone())
            }
            Err(err) => {
                log::error!("Error: {}", err);
                Err(err)
            }
        },
        None => {
            log::error!("Command not found {}", input.get_name());
            Err(AppError::CommandNotFound(input.get_name().to_owned()))
        }
    }
}

#[async_trait]
pub trait Command {
    fn get_name(&self) -> &str;

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError>;

    fn can_handle(&self, input: &CommandInput) -> bool {
        self.get_name() == input.get_name()
    }
}

pub struct CommandArg {
    name: String,
    value: String,
}

pub struct Parameters {
    override_params: Vec<CommandArg>,
    params: Vec<CommandArg>,
    default_params: Vec<CommandArg>,
}

impl Parameters {
    fn empty() -> Self {
        Parameters {
            override_params: Vec::new(),
            params: Vec::new(),
            default_params: Vec::new(),
        }
    }

    fn new(
        override_params: Vec<CommandArg>,
        params: Vec<CommandArg>,
        default_params: Vec<CommandArg>,
    ) -> Self {
        Parameters {
            override_params,
            params,
            default_params,
        }
    }
}

pub struct CommandInput {
    name: String,
    crypto_key: Option<String>,
    ipaddress: Option<IpAddr>,
    action_args: Vec<CommandArg>,
    parameters: Parameters,
    credentials: Vec<Credential>,
}

impl CommandInput {
    pub fn new(
        name: &str,
        crypto_key: Option<&str>,
        ipaddress: Option<IpAddr>,
        action_args: Vec<CommandArg>,
        parameters: Parameters,
        credentials: Vec<Credential>,
    ) -> Self {
        CommandInput {
            name: name.to_owned(),
            crypto_key: crypto_key.map(|c| c.to_owned()),
            ipaddress: ipaddress.map(|i| i.to_owned()),
            action_args,
            parameters,
            credentials,
        }
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_ipaddress(&self) -> Option<IpAddr> {
        self.ipaddress
    }
    fn find_single_arg(&self, name: &str) -> Result<&str, AppError> {
        self.action_args
            .iter()
            .find(|a| a.name == name)
            .map(|a| a.value.as_str())
            .ok_or_else(|| AppError::ArgumentNotFound(name.to_owned()))
    }
    fn find_all_args(&self, name: &str) -> Result<Vec<&str>, AppError> {
        Ok(self
            .action_args
            .iter()
            .filter(|a| a.name == name)
            .map(|a| a.value.as_str())
            .collect())
    }

    fn find_param(&self, name: &str) -> Result<&str, AppError> {
        let override_param = self
            .parameters
            .override_params
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_str());

        if let Some(override_param) = override_param {
            return Ok(override_param);
        }
        let param = self
            .parameters
            .params
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_str());

        if let Some(param) = param {
            return Ok(param);
        }

        self.parameters
            .default_params
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_str())
            .ok_or_else(|| AppError::ArgumentNotFound(name.to_owned()))
    }
    fn find_credential(&self, name: &str) -> Result<Credential, AppError> {
        self.credentials
            .iter()
            .find(|c| c.name == name)
            .map(|c| c.to_owned())
            .ok_or_else(|| AppError::CredentialNotFound(name.to_owned()))
    }
}

pub trait CommandResult {}
