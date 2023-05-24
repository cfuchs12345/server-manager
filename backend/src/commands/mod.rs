use std::pin::Pin;

use lazy_static::lazy_static;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::models::{error::AppError, server::{Credential, Server, Feature}, plugin::{Plugin, action::Action, data::Data}};

mod http;
mod wol;
mod sleep;
mod socket;
mod common;
pub mod ping;




lazy_static!{
    
    static ref COMMANDS: RwLock<Vec<Box<dyn Command + Sync + Send >>> = RwLock::new(get_command_list());
}

fn get_command_list() -> Vec<Box<dyn Command + Sync + Send>>{
    let mut list:Vec<Box<dyn Command + Sync + Send>> = Vec::new();
    list.push(Box::new(http::HttpCommand::new()));
    list.push(Box::new(wol::WoLCommand::new()));
    list
}

pub async fn execute(input: &CommandInput) -> Result<Pin<Box<dyn CommandResult>>, AppError> {
    let commands = COMMANDS.try_read().unwrap();
    
    match commands.iter().find(|c| c.can_handle(input)) {
        Some(cmd) => {
            match cmd.execute(input).await {
                Ok(res) => {
                    Ok(res)
                },
                Err(err) => {
                    log::error!("Error: {}", err);
                    Err(err)
                }
            }
        }
        None => {
            log::error!("Command not found {}", input.get_name());
            Err(AppError::CommandNotFound(input.get_name().to_owned()))
        }
    }
}


#[async_trait]
pub trait Command {
    fn get_name(&self) -> &str;

    async fn execute(&self, input: &CommandInput) -> Result<Pin<Box<dyn CommandResult>>, AppError>;

    fn can_handle(&self, input: &CommandInput) -> bool {
        self.get_name() == input.get_name()
    }
}

pub struct CommandArg {
    name: String,
    value: String
}



pub struct CommandInput {
    name: String,
    crypto_key: String,
    ipaddress: Option<String>,
    action_args: Vec<CommandArg>,
    params_override: Vec<CommandArg>,
    params: Vec<CommandArg>,
    params_default: Vec<CommandArg>,
    credentials: Vec<Credential>
}

impl CommandInput {    
    pub fn new(name: &str, crypto_key: &str, ip_address: Option<&str>, action_args: Vec<CommandArg>, params_override: Vec<CommandArg>, params: Vec<CommandArg>,  params_default: Vec<CommandArg>, credentials: Vec<Credential>) -> Self {
        CommandInput { name: name.to_owned(), crypto_key: crypto_key.to_owned(), ipaddress: ip_address.map(|i| i.to_owned()), action_args, params_override, params, params_default, credentials }
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_ipaddress(&self) -> Option<String> {
        self.ipaddress.clone()
    }
    fn find_single_arg(&self, name: &str) -> Result<&str, AppError> {
        self.action_args.iter().find(|a| a.name == name).map(|a| a.value.as_str()).ok_or_else(|| AppError::ArgumentNotFound(name.to_owned()))
    }
    fn find_all_args(&self, name: &str) -> Result<Vec<&str>, AppError> {
        Ok(self.action_args.iter().filter(|a| a.name == name).map(|a| a.value.as_str()).collect())
    }

    fn find_param(&self, name: &str) -> Result<&str, AppError> {
        let override_param = self.params_override.iter().find(|p| p.name == name).map(|p| p.value.as_str());

        if override_param.is_some() {
            return Ok(override_param.unwrap());
        }
        let param = self.params.iter().find(|p| p.name == name).map(|p| p.value.as_str());

        if param.is_some() {
            return Ok(param.unwrap());
        }

        self.params_default.iter().find(|p| p.name == name).map(|p| p.value.as_str())       
        .ok_or_else(|| AppError::ArgumentNotFound(name.to_owned()))
    }    
    fn find_credential(&self, name: &str) -> Result<Credential, AppError> {
        self.credentials.iter().find(|c| c.name == name).map(|c| c.to_owned()).ok_or_else(|| AppError::CredentialNotFound(name.to_owned()))
    }
}



pub fn make_command_input_from_subaction(command: &str, server: &Server, crypto_key: &str, action: &Action, action_params: Option<&str>, feature: &Feature, plugin: &Plugin  ) -> CommandInput  {
    CommandInput::new(command, crypto_key, Some(server.ipaddress.as_str()), action_args_to_command_args(action), action_params_to_command_args(action_params), feature_params_to_command_args(feature),  plugin_default_params_to_command_args(plugin), feature.credentials.clone() )
}

pub fn make_command_input_from_data(command: &str, server: &Server, crypto_key: &str, data: &Data, action_params: Option<&str>, feature: &Feature, plugin: &Plugin  ) -> CommandInput  {
    CommandInput::new(command, crypto_key, Some(server.ipaddress.as_str()), data_args_to_command_args(data), action_params_to_command_args(action_params), feature_params_to_command_args(feature),  plugin_default_params_to_command_args(plugin), feature.credentials.clone() )
}

fn data_args_to_command_args(data: &Data) -> Vec<CommandArg> {
    data.args.iter().map(|a| CommandArg { name: a.arg_type.clone(), value: a.value.clone()}).collect()
}

fn action_args_to_command_args(action: &Action) -> Vec<CommandArg> {
    action.args.iter().map(|a| CommandArg { name: a.arg_type.clone(), value: a.value.clone()}).collect()
}

fn plugin_default_params_to_command_args(plugin: &Plugin) -> Vec<CommandArg> {
    plugin.params.iter().map(|p| CommandArg { name: p.name.clone(), value: p.default_value.clone()}).collect()
}

fn feature_params_to_command_args(feature: &Feature) -> Vec<CommandArg>  {
    feature.params.iter().map( |p| CommandArg { name: p.name.clone(), value: p.value.clone()}).collect()
}

fn action_params_to_command_args(action_params: Option<&str>) -> Vec<CommandArg> {
    let mut list = Vec::new();

    if let Some(action_params) = action_params {
        let split = action_params.split(',');

        for str in split {
            let single_param = str.split_at(str.find('=').unwrap());
            
            list.push(CommandArg {
                name: single_param.0.to_owned(),
                value: single_param.1[1..].to_owned() // skip the first char which is still the separator
            });
        }
    }
    list
}





pub trait CommandResult {
    fn get_result(&self) -> Option<String>;
}