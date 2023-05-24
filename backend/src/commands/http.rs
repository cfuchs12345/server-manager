use super::{common::replace, Command, CommandInput, CommandResult};
use crate::models::{error::AppError};
use async_trait::async_trait;
use std::{pin::Pin};

#[derive(Clone)]
pub struct HttpCommand {
    name: String,
}

impl HttpCommand {
    pub fn new() -> Self {
        HttpCommand {
            name: "http".to_string(),
        }
    }
}

#[async_trait]
impl Command for HttpCommand {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn execute(&self, input: &CommandInput) -> Result<Pin<Box<dyn CommandResult>>, AppError> {
        let url = input
            .find_single_arg("url")?;
        let method = input
            .find_single_arg("method")?;
        let headers = input
            .find_all_args("header")?;

        let body: &str = match method {
            "post" =>  input.find_single_arg("body").unwrap_or( {
                log::warn!("Actually expected a body for a post request. Continuing with an empty body.");
                ""
            }),
            "put" => input.find_single_arg("body").unwrap_or( {
                    log::error!("Actually expected a body for a put request. Continuing with an empty body.");
                    ""
            }),
            _ => "",
        };

        let normal_and_masked_url: (String, String) =
            replace::replace(url, input)?;
        let normal_and_masked_body: (String, String) =
            replace::replace(body, input)?;
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

        match crate::common::execute_http_request(
            normal_and_masked_url.0,
            method,
            Some(normal_and_replaced_headers),
            Some(normal_and_masked_body.0),
        )
        .await {
            Ok(response_string) => {
                log::info!("response {}", response_string);
                Ok(Box::pin(HttpCommandResult::new(Some(response_string))))
            }
            Err(err) => {
                log::error!("Error: {}", err);
                Err(AppError::from(err))
            }
        }
        

       
    }
}

struct HttpCommandResult {
    result: Option<String>
}

impl HttpCommandResult {
    fn new(result: Option<String>) -> Self {
        HttpCommandResult {
            result
        }
    }
}

impl CommandResult for HttpCommandResult {
    fn get_result(&self) -> Option<String> {
        self.result.clone()
    }
}
