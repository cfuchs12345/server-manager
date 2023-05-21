use std::pin::Pin;
use async_trait::async_trait;
use crate::models::{input::ActionOrDataInput, error::AppError};
use super::{common::replace, Command, CommandResult,  CommandInput};

pub struct HttpCommand {

}

#[async_trait]
impl Command for HttpCommand {
    fn new() -> Self {
        HttpCommand {            
        }
    }

    fn get_name(&self) -> &str {
        "http"
    }

    async fn execute (&self, input: CommandInput ) -> Pin<Box<dyn CommandResult>> {
        //let res = execute_http_command(ipaddress, input).await;

       //execute_http_request(url, method, headers, body)

        Box::pin(HttpCommandResult::new())
    }
}

struct HttpCommandResult {
}

impl HttpCommandResult {
    fn new() -> Self {
        HttpCommandResult {  }
    }
}

impl CommandResult for HttpCommandResult {
    fn get_result(&self) -> Option<&str> {
        None
    }

    fn get_error_message(&self) -> Option<&str> {
        None
    }

    fn is_error(&self) -> bool {
        false
    }
}


pub async fn execute_http_command<'a>(
    ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<Option<String>, AppError> {
    let url = input
        .find_arg("url")
        .ok_or(AppError::MissingArgument("url".to_string()))?;
    let method = input
        .find_arg("method")
        .ok_or(AppError::MissingArgument("method".to_string()))?;
    let headers = input
        .find_all_args("header")
        .iter()
        .map(|argdef| argdef.value.clone())
        .collect();

    let body: &str = match method.value.as_str() {
        "post" => {
            match input.find_arg("body") {
                Some(arg) => arg.value.as_str(),
                None => {
                    log::warn!("Actually expected a body for a post request. Continuing with an empty body.");
                    ""
                }
            }
        }
        "put" => {
            match input.find_arg("body") {
                Some(arg) => arg.value.as_str(),
                None => {
                    log::error!("Actually expected a body for a put request. Continuing with an empty body.");
                    ""
                }
            }
        }
        _ => "",
    };

    let normal_and_masked_url: (String, String) = replace::replace(url.value.clone(), &ipaddress, input);
    let normal_and_masked_body: (String, String) = replace::replace(body.to_string(), &ipaddress, input);
    let normal_and_replaced_headers: Vec<(String, String)> =
        replace::replace_list(headers, &ipaddress, input);

    if !body.is_empty() {
        log::debug!(
            "About to execute method {} on url {} with body {}",
            method.value,
            normal_and_masked_url.1,
            normal_and_masked_body.1
        );

        log::debug!(
            "About to execute method {} on url {} with body {}",
            method.value,
            normal_and_masked_url.0,
            normal_and_masked_body.0
        );
    } else {
        log::debug!(
            "About to execute method {} on url {}",
            method.value,
            normal_and_masked_url.1
        );

        log::debug!(
            "About to execute method {} on url {}",
            method.value,
            normal_and_masked_url.0
        );
    }

    if normal_and_masked_url.0.trim().is_empty() {
        log::warn!(
            "Given url is empty after replacing placeholders. Was before replace: {}. Request will not be executed",
            url.value
        );
        return Ok(None);
    }

    let text = crate::common::execute_http_request(
        normal_and_masked_url.0,
        method.value.as_str(),
        Some(normal_and_replaced_headers),
        Some(normal_and_masked_body.0),
    )
    .await
    .unwrap_or_default();

    Ok(Some(text))
}