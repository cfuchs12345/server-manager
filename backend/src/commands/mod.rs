use std::pin::Pin;

use lazy_static::lazy_static;
use async_trait::async_trait;

use crate::models::{input::ActionOrDataInput, error::AppError};

mod http;
mod wol;
mod sleep;
mod socket;
mod common;
pub mod ping;

lazy_static!{
    static ref HTTP_COMMAND:http::HttpCommand = http::HttpCommand::new();
}


pub async fn execute_command<'a>(
    ipaddress: Option<String>,
    input: &ActionOrDataInput,
) -> Result<Option<String>, AppError> {
    match input.command.as_str() {
        "http" => http::execute_http_command(ipaddress.unwrap(), input).await,
        "wol" => wol:: execute_wol_command(ipaddress.unwrap(), input).await,
        //"ping_all" => ping::ping_all().await,
        //"ping" => ping::status_check(ips_to_check, use_cache).await,
        y => {
            let error = format!("Action command {} is not implemented ", y);
            log::error!("{}", error);
            Err(AppError::InvalidArgument("command".to_string(), Some(y.to_string())))
        }
    }
}

pub async fn execute(input: CommandInput) -> Pin<Box<dyn CommandResult>> {

    if input.get_name() == HTTP_COMMAND.get_name() {
        return HTTP_COMMAND.execute(input).await;
    }

    Box::pin(ErrorCommandResult::new("Command not found"))
}


#[async_trait]
pub trait Command {
    fn new() -> Self;
    fn get_name(&self) -> &str;
    async fn execute(&self, input: CommandInput) -> Pin<Box<dyn CommandResult>>;
}

pub struct CommandInput {
    name: String
}

impl CommandInput {    
    fn new(name: &str) -> Self {
        CommandInput { name: name.to_owned() }
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub trait CommandResult {
    fn get_result(&self) -> Option<&str>;
    fn is_error(&self) -> bool;
    fn get_error_message(&self) -> Option<&str>;
}

struct ErrorCommandResult {
    is_error: bool,
    error_message: String,
}

impl ErrorCommandResult {
    #[allow(dead_code)]
    pub fn new(message: &str) -> Self {  
        ErrorCommandResult {
            is_error: true,
            error_message: message.to_owned()
        }
    }
}
impl CommandResult for ErrorCommandResult {    
    fn get_error_message(&self) -> Option<&str> {
        Some(self.error_message.as_str())
    }

    fn get_result(&self) -> Option<&str> {
        None
    }

    fn is_error(&self) -> bool {
        self.is_error
    }
}