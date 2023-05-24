use std::pin::Pin;

use async_trait::async_trait;
use mac_address::MacAddress;

use crate::models::{error::AppError};

use super::{Command, CommandInput, CommandResult};

#[derive(Clone)]
pub struct WoLCommand {
    name: String,
}

impl WoLCommand {
    pub fn new() -> Self {
        WoLCommand {
            name: "wol".to_string(),
        }
    }
}

#[async_trait]
impl Command for WoLCommand {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn execute(&self, input: &CommandInput) -> Result<Pin<Box<dyn CommandResult>>, AppError> {
        let feature_param = input.find_param("mac_address")?;

        let address = feature_param.parse::<MacAddress>().map_err(|_| {
            AppError::InvalidArgument("mac_address".to_string(), Some(feature_param.to_owned()))
        })?;

        let magic_packet = wake_on_lan::MagicPacket::new(&address.bytes());

        match magic_packet.send() {
            Ok(_success) => {
                log::debug!(
                    "Successfully send magic packet to host with mac address {}",
                    address
                );
                Ok(Box::pin(WolCommandResult::new(Some("SENT".to_string()))))
            }
            Err(err) => {
                log::error!(
                    "Could not send magic packet due to technical problems: {:?}",
                    err
                );
                Err(AppError::Unknown(Box::new(err)))
            }
        }
    }
}


struct WolCommandResult {
    result: Option<String>
}

impl WolCommandResult {
    fn new(result: Option<String>) -> Self {
        WolCommandResult {
            result
        }
    }
}

impl CommandResult for WolCommandResult {
    fn get_result(&self) -> Option<String> {
        self.result.clone()
    }
}