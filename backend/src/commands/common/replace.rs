use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::{input::ActionOrDataInput, server::Credential};



/// This enum hides the actual regular expressions and the matching and provides methods for
/// * easy extraction of matched strings
/// * strip of the markers and returning the actual name of the placeholder
enum Placeholder {
    Param,
    Credential,
    Base64,
}


lazy_static! {
    static ref PARAM_REGEX: Regex = Regex::new(Placeholder::Param.get_pattern()).unwrap();
    static ref CREDENTIAL_REGEX: Regex = Regex::new(Placeholder::Credential.get_pattern()).unwrap();
    static ref BASE64_REGEX: Regex = Regex::new(Placeholder::Base64.get_pattern()).unwrap();
    
}



impl Placeholder {
    fn get_pattern(&self) -> &str {
        match self {
            Placeholder::Param => r"(\$\{params\..*?\})",
            Placeholder::Credential => r"(\$\{credentials\..*?\})",
            Placeholder::Base64 => r"(\$\{encode_base64\(.*?\)\})",
        }
    }

    pub fn extract_placeholders(&self, input: String) -> Vec<String> {
        let matches = match self {
            Placeholder::Param => PARAM_REGEX.find_iter(input.as_str()),
            Placeholder::Credential => CREDENTIAL_REGEX.find_iter(input.as_str()),
            Placeholder::Base64 => BASE64_REGEX.find_iter(input.as_str()),
        };

        matches.map(|mat| mat.as_str().to_owned()).collect()
    }

    pub fn strip_of_marker(&self, value: &str) -> String {
        match self {
            Placeholder::Param => value.replace("${params.", "").replace('}', ""),
            Placeholder::Credential => value.replace("${credentials.", "").replace('}', ""),
            Placeholder::Base64 => value.replace("${encode_base64(", "").replace(")}", ""),
        }
    }
}



pub fn replace_list(
    input_strings: Vec<String>,
    ipaddress: &str,
    input: &ActionOrDataInput,
) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];

    for input_string in input_strings {
        let res = replace(input_string, ipaddress, input);
        result.push(res);
    }

    result
}
pub fn replace(input_string: String, ipaddress: &str, input: &ActionOrDataInput) -> (String, String) {
    let mut result: String = input_string;
    let mut masked: String;

    result = result.replace("${IP}", ipaddress);
    result = replace_param(result, input);
    let both: (String, String) = replace_credentials(result, input); // we now have two string - the unmasked and the masked which can be logged for example
    result = both.0;
    masked = both.1;
    result = replace_base64_encoded(result); // base 64 encode should happen on both idependently
    masked = replace_base64_encoded(masked); // actually the base 64 encoded masked version outputs an incorrect encoded value

    (result, masked)
}

fn replace_param(input_string: String, input: &ActionOrDataInput) -> String {
    let mut result = input_string.clone();

    for placeholder in Placeholder::Param.extract_placeholders(input_string) {
        let name = Placeholder::Param.strip_of_marker(&placeholder);

        let replacement_option = input.find_param(name.as_str());

        if let Some(replacement) = replacement_option {
            result = result.replace(placeholder.as_str(), replacement.value.as_str());
        } else {
            log::error!("Found no replacement for placeholder {}", placeholder);
        }
    }
    result
}

fn replace_credentials(input_string: String, input: &ActionOrDataInput) -> (String, String) {
    let mut result = input_string.clone();
    let mut masked = input_string.clone();

    for placeholder in Placeholder::Credential.extract_placeholders(input_string) {
        let name = Placeholder::Credential.strip_of_marker(&placeholder);

        let replacement = get_credential_value(name.as_str(), input);

        if let Some(replacement_tuple) = replacement {
            result = result.replace(placeholder.as_str(), replacement_tuple.0.as_str());
            if replacement_tuple.1 {
                masked = masked.replace(placeholder.as_str(), "******");
            } else {
                masked = masked.replace(placeholder.as_str(), replacement_tuple.0.as_str());
            }
        } else {
            log::error!("Found no replacement for placeholder {}", placeholder);
        }
    }
    (result, masked)
}

fn replace_base64_encoded(input: String) -> String {
    let mut result = input.clone();

    for placeholder in Placeholder::Base64.extract_placeholders(input) {
        let to_encode = Placeholder::Base64.strip_of_marker(&placeholder);

        let replacement = encode_base64(&to_encode);

        result = result.replace(placeholder.as_str(), replacement.as_str());
    }

    result
}

fn get_credential_value(name: &str, input: &ActionOrDataInput) -> Option<(String, bool)> {
    input
        .find_credential(name)
        .map(|credential| (decrypt(credential, input.crypto_key.as_ref().unwrap()), credential.encrypted))
}

fn decrypt(credential: &Credential, crypto_key: &str) -> String {
    if credential.encrypted {
        crate::common::default_decrypt(&credential.value, crypto_key)
    } else {
        credential.value.clone()
    }
}

fn encode_base64(placeholder: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(Placeholder::Base64.strip_of_marker(placeholder))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_param_extract() {
        assert_eq!(
            Placeholder::Param
                .extract_placeholders("test ${params.test}".to_string())
                .len(),
            1
        );
        assert_ne!(
            Placeholder::Param
                .extract_placeholders("test params.test".to_string())
                .len(),
            1
        );
        assert_eq!( Placeholder::Param.extract_placeholders("${params.protocol}://${credentials.username}:${credentials.password}192.168.178.20:${params.port}/${params.command}".to_string()).len(), 3);

        assert_eq!(
            Placeholder::Base64
                .extract_placeholders("${encode_base64(USERNAME)}".to_string())
                .first()
                .unwrap()
                .to_owned(),
            "${encode_base64(USERNAME)}".to_owned()
        );
    }

    #[test]
    fn test_regex_strip_of_marker() {
        assert_eq!(
            Placeholder::Param.strip_of_marker(&"${params.test}".to_string()),
            "test"
        );
        assert_eq!(
            Placeholder::Base64.strip_of_marker(&"${encode_base64(USERNAME)}".to_string()),
            "USERNAME"
        );
    }

    #[test]
    fn test_encode_bas64() {
        assert_eq!(encode_base64(&"USERNAME".to_string()), "VVNFUk5BTUU");
        assert_eq!(encode_base64(&"test:test".to_string()), "dGVzdDp0ZXN0");
    }
}