use base64::{engine,engine::general_purpose, Engine as _, alphabet};
use rand::{rngs::OsRng, RngCore};

mod crypt;
mod http_functions;
mod script_languages;
mod mail;
mod onetimekey;

pub use http_functions::GET;
pub use http_functions::POST;
pub use http_functions::DELETE;

pub use http_functions::create_http_client;
pub use http_functions::execute_http_request;
pub use crypt::default_encrypt;
pub use crypt::default_decrypt;
pub use crypt::aes_decrypt;
pub use crypt::make_aes_secrect;
pub use crypt::get_random_key32;
pub use crypt::hash_password;
pub use crypt::verify_password;

pub use script_languages::match_with_lua;
pub use script_languages::match_with_rhai;

pub use mail::is_smtp_config_valid;
pub use mail::send_email;

pub use onetimekey::OneTimeKey;
pub use onetimekey::invalidate_expired_one_time_keys;


const URLSAFE_WITH_PAD: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);



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


pub fn decode_base64_urlsafe_with_pad(str: &str) -> String {
    String::from_utf8(URLSAFE_WITH_PAD.decode(str).unwrap()).unwrap()
}








#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_base64() {
        assert_eq!(encode_base64(&"USERNAME".to_string()), "VVNFUk5BTUU");
        assert_eq!(encode_base64(&"test:test".to_string()), "dGVzdDp0ZXN0");
        assert_eq!(encode_base64(&"123:U2FsdGVkX1+c+mor0/ctcOn3K4/MU9yZv56ZzSfqdxs=".to_string()), "MTIzOlUyRnNkR1ZrWDErYyttb3IwL2N0Y09uM0s0L01VOXladjU2WnpTZnFkeHM9");        
    }

    #[test]
    fn test_dencode_base64() {                                   
        assert_eq!(decode_base64_urlsafe_with_pad(&"MTIzOlUyRnNkR1ZrWDErYyttb3IwL2N0Y09uM0s0L01VOXladjU2WnpTZnFkeHM9".to_string()), "123:U2FsdGVkX1+c+mor0/ctcOn3K4/MU9yZv56ZzSfqdxs=");   
        
    }    
}
