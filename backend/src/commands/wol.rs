use mac_address::MacAddress;

use crate::models::{input::ActionOrDataInput, error::AppError};



pub async fn execute_wol_command<'a>(
    _ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<Option<String>, AppError> {
    let feature_param = input.find_param("mac_address");
    match feature_param {
        Some(found_feature_param) => match found_feature_param.value.parse::<MacAddress>() {
            Ok(address) => {
                let magic_packet = wake_on_lan::MagicPacket::new(&address.bytes());

                match magic_packet.send() {
                    Ok(_success) => {
                        log::debug!(
                            "Successfully send magic packet to host with mac address {}",
                            address
                        );
                        Ok(Some("SEND".to_string()))
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
            Err(err) => {
                log::error!(
                    "Given mac address {} is invalid. Cannot send magic packet for WoL {}",
                    &found_feature_param.value,
                    err
                );
                Err(AppError::InvalidArgument("mac_address".to_string(), Some(found_feature_param.value.clone())))
            }
        },
        None => Ok(None),
    }
}

