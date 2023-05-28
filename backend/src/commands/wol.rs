use std::any::Any;

use async_trait::async_trait;
use mac_address::MacAddress;

use crate::models::{error::AppError, server::Feature};

use super::{common, Command, CommandInput, CommandResult};

pub const WOL: &str = "wol";

#[derive(Clone)]
pub struct WoLCommand {}

impl WoLCommand {
    pub fn new() -> Self {
        WoLCommand {}
    }
}

#[async_trait]
impl Command for WoLCommand {
    fn get_name(&self) -> &str {
        WOL
    }

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError> {
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
                Ok(Box::new(WolCommandResult::new()))
            }
            Err(err) => {
                log::error!(
                    "Could not send magic packet due to technical problems: {:?}",
                    err
                );
                Err(AppError::Unknown(format!("{}", err)))
            }
        }
    }
}

#[derive(Clone)]
pub struct WolCommandResult {}

impl CommandResult for WolCommandResult {}

impl WolCommandResult {
    fn new() -> Self {
        WolCommandResult {}
    }

    pub fn get_result(&self) -> bool {
        true
    }
}

pub fn make_input(feature: &Feature) -> CommandInput {
    let params = super::Parameters::new(
        Vec::new(),
        common::params_to_command_args(&feature.params),
        Vec::new(),
    );

    CommandInput::new(WOL, None, None, Vec::new(), params, Vec::new())
}
