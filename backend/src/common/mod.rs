use base64::{alphabet, engine, engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};

mod crypt;
mod http_functions;
mod mail;
mod onetimekey;
mod script_languages;

pub use http_functions::DELETE;
pub use http_functions::GET;
pub use http_functions::POST;

pub use crypt::aes_decrypt;
pub use crypt::default_decrypt;
pub use crypt::default_encrypt;
pub use crypt::get_random_key32;
pub use crypt::hash_password;
pub use crypt::make_aes_secrect;
pub use crypt::verify_password;
pub use http_functions::execute_http_request;
#[cfg(all(target_os = "linux"))]
pub use http_functions::execute_socket_request;

pub use script_languages::match_with_lua;
pub use script_languages::match_with_rhai;
pub use script_languages::process_with_lua;
pub use script_languages::process_with_rhai;

pub use mail::is_smtp_config_valid;
pub use mail::send_email;

pub use onetimekey::invalidate_expired_one_time_keys;
pub use onetimekey::OneTimeKey;

use crate::models::error::AppError;

const URLSAFE_WITH_PAD: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

pub const IDENTIFIER: &str = "Identifier";
pub const SUB_IDENTIFIER: &str = "Sub_Identifier";
pub const SUB_IDENTIFIER_2: &str = "Sub_Identifier2";
pub const VALUE: &str = "Value";
pub const TIMESTAMP: &str = "timestamp";

pub fn generate_long_random_string() -> String {
    let key: String = {
        let mut buff = [0_u8; 128];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };
    key
}

pub fn generate_short_random_string() -> String {
    let key: String = {
        let mut buff = [0_u8; 8];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };
    key
}

pub fn encode_base64(str: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(str)
}

pub fn decode_base64_urlsafe_with_pad(str: &str) -> Result<String, AppError> {
    String::from_utf8(URLSAFE_WITH_PAD.decode(str)?).map_err(AppError::from)
}

pub fn convert_value_to_str(value: &serde_json::Value) -> Option<String> {
    if value.is_f64() {
        value.as_f64().map(|n| n.to_string())
    } else if value.is_i64() {
        value.as_i64().map(|n| n.to_string())
    } else if value.is_u64() {
        value.as_u64().map(|n| n.to_string())
    } else if value.is_boolean() {
        value.as_bool().map(|b| b.to_string())
    } else if value.is_string() {
        value.as_str().map(|s| s.to_owned())
    } else {
        log::trace!("Unhandled json type for value {:?}", value);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_base64() {
        assert_eq!(encode_base64("USERNAME"), "VVNFUk5BTUU");
        assert_eq!(encode_base64("test:test"), "dGVzdDp0ZXN0");
        assert_eq!(
            encode_base64("123:U2FsdGVkX1+c+mor0/ctcOn3K4/MU9yZv56ZzSfqdxs="),
            "MTIzOlUyRnNkR1ZrWDErYyttb3IwL2N0Y09uM0s0L01VOXladjU2WnpTZnFkeHM9"
        );
    }

    #[test]
    fn test_dencode_base64() {
        assert_eq!(
            decode_base64_urlsafe_with_pad(
                "MTIzOlUyRnNkR1ZrWDErYyttb3IwL2N0Y09uM0s0L01VOXladjU2WnpTZnFkeHM9"
            )
            .expect("should not happen"),
            "123:U2FsdGVkX1+c+mor0/ctcOn3K4/MU9yZv56ZzSfqdxs="
        );
    }
}
