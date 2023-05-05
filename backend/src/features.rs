use crate::{conversion, crypt, http_functions};
use crate::plugin_types::{Data, DependsDef, Plugin};
use crate::server_types::{Feature, Server, Credential};
use crate::types::ActionOrDataInput;
use std::io::ErrorKind;
use actix_web::{Error};
use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use mac_address::MacAddress;
use regex::Regex;

use rlua::Lua;
use rhai::{Engine, Scope};



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

/// Executes a defined action of a plugin on the given server
/// # Arguments
/// 
/// * `server` - the server struct representing the server on which the action should be executed 
/// * `plugin` - the plugin to which the action query belongs to
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `action_id`- the identifier of the action
/// * `accept_self_signed_certificates` - boolean flag if the server should accept self-signed SSL certiticates
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_action(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    action_id: &str,
    accept_self_signed_certificates: bool,
    crypto_key: String
) -> Result<bool, Error> {
    match plugin.find_action(action_id) {
        Some(plugin_action) => {
            let input: ActionOrDataInput = ActionOrDataInput::get_input_from_action(
                plugin_action,
                plugin,
                feature,
                accept_self_signed_certificates,
                crypto_key
            );

            execute_command(server.ipaddress.clone(), &input)
                .await
                .map(|_| true)
        }
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, plugin.id);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}

/// Executes all data queries on the given server for all given plugins
/// # Arguments
/// 
/// * `server` - the server struct representing the server on which the query should be executed
/// * `plugins` - a vector of plugins for which the data queries should be executed
/// * `accept_self_signed_certificates` - boolean flag if the server should accept self-signed SSL certiticates
/// * `template_engine` - the template engine struct that is used to render the output in a readable format
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_data_query(
    server: &Server,
    plugins: &[Plugin],
    accept_self_signed_certificates: bool,
    template_engine: &handlebars::Handlebars<'static>,
    crypto_key: String
) -> Result<Vec<String>, Error> {
    let mut results: Vec<String> = vec![];

    let tuples: Vec<(&Feature, &Plugin)> = server
        .features
        .iter()
        .filter_map(|f| Some((f, find_plugin_for_feature(f, plugins)?)))
        .collect();

    for tuple in tuples {
        for data in &tuple.1.data {
            let res = execute_specific_data_query(
                server,
                tuple.1,
                tuple.0,
                data,
                accept_self_signed_certificates,
                crypto_key.clone()
            )
            .await?;

            if !data.template.is_empty() {
                let result = conversion::convert_json_to_html(
                    data.template.as_str(),
                    res,
                    template_engine,
                    data,
                )?;
                results.push(result);
            } else {
                results.push(res);
            }
        }
    }

    Ok(results)
}

/// Executes a specific data query on the given server for a given data point config of a plugin
/// # Arguments
/// 
/// * `server` - the server struct representing the server on which the query should be executed
/// * `plugin` - the plugin to which the data query belongs to
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `data` - the actual data query (as defined in the plugin) that should be executed
/// * `accept_self_signed_certificates` - boolean flag if the server should accept self-signed SSL certiticates
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_specific_data_query(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    data: &Data,
    accept_self_signed_certificates: bool,
    crypto_key: String
) -> Result<String, Error> {
    let input: ActionOrDataInput = ActionOrDataInput::get_input_from_data(
        data,
        plugin,
        feature,
        accept_self_signed_certificates,
        crypto_key
    );

    execute_command(server.ipaddress.clone(), &input).await
}

/// Checks if all conditions that are defined for an action of a plugin are met and that it can be executed by the user
/// # Arguments
/// 
/// * `server` - the server struct representing the server on which the query should be executed
/// * `plugin` - the plugin to which the data query belongs to
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `action_id`- the identifier of the action
/// * `accept_self_signed_certificates` - boolean flag if the server should accept self-signed SSL certiticates
/// * `persistence` - the persistence struct that helps to interact with the underlying database

pub async fn check_condition_for_action_met(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    action_id: &str,
    accept_self_signed_certificates: bool,
    crypto_key: String
) -> Result<bool, Error> {
    match plugin.find_action(action_id) {
        Some(action) => {
            let status = crate::status::status_check(vec![server.ipaddress.clone()], true).await?;

            let mut res = match action.available_for_state {
                crate::plugin_types::State::Active => {
                    match status.first() {
                        Some(status) => status.is_running,
                        None => false, // unknown state - better do not allow an action
                    }
                }
                crate::plugin_types::State::Inactive => {
                    match status.first() {
                        Some(status) => !status.is_running,
                        None => false, // unknown state - better do not allow an action
                    }
                }
                crate::plugin_types::State::Any => {
                    // no status check - just allow it
                    true
                }
            };

            if !res {
                // check if status dependency already failed - early exit
                return Ok(res);
            }

            if let Some(status) = status.first() {
                    if status.is_running { // if not running, no need to start any request
                        // now check data dependencies one by one
                        for depends in &action.depends {
                            match find_data_for_action_depency(depends, plugin) {
                                Some(data) => {
                                    let response = execute_specific_data_query(
                                        server,
                                        plugin,
                                        feature,
                                        data,
                                        accept_self_signed_certificates,
                                        crypto_key.clone()
                                    )
                                    .await?;

                                    res &= response_data_match(depends, response.as_str())?;

                                    if !res {
                                        log::info!("Depencies for data {} of plugin {} for server {} not met .Reasponse was {}", data.id, plugin.id, server.ipaddress, response);
                                        break;
                                    }
                                }
                                None => {
                                    let error = format!(
                                        "dependent data with id  {} not found for action {}",
                                        depends.data_id, action_id
                                    );
                                    log::error!("{}", error);
                                    res = false;
                                    break;
                                }
                            }
                        }
                    }
                    else if !action.depends.is_empty() {
                        res = false;
                    }               
            };

            Ok(res)
        }
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, plugin.id);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}

async fn execute_command<'a>(ipaddress: String, input: &ActionOrDataInput) -> Result<String, Error> {
    match input.command.as_str() {
        "http" => execute_http_command(ipaddress, input).await,
        "wol" => execute_wol_command(ipaddress, input).await,
        y => {
            let error = format!("Action command {} is not implemented ", y);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}

async fn execute_http_command<'a>(
    ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<String, Error> {
    let url = input
        .find_arg("url")
        .ok_or(Error::from(std::io::Error::new(
            ErrorKind::Other,
            "url not found",
        )))?;
    let method = input
        .find_arg("method")
        .ok_or(Error::from(std::io::Error::new(
            ErrorKind::Other,
            "method not found",
        )))?;
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
                    log::error!("Actually expected a body for a post request. Continuing with an empty body.");
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

    let normal_and_masked_url: (String, String) = replace(url.value.clone(), &ipaddress, input);
    let normal_and_masked_body: (String, String) = replace(body.to_string(), &ipaddress, input);
    let normal_and_replaced_headers: Vec<(String, String)> =
        replace_list(headers, &ipaddress, input);

    if !body.is_empty() {
        log::debug!(
            "About to execute method {} on url {} with body {}",
            method.value,
            normal_and_masked_url.1,
            normal_and_masked_body.1
        );

        log::info!(
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

    match http_functions::execute_http_request(
        normal_and_masked_url.0,
        method.value.as_str(),
        Some(normal_and_replaced_headers),
        Some(normal_and_masked_body.0),
        input.accept_self_signed_ceritificates,
    )
    .await
    {
        Ok(response) => {
            let text = response.text().await.unwrap_or_default();
            log::debug!(
                "Response for http request to url {} was: {}",
                normal_and_masked_url.1,
                text
            );
            Ok(text)
        }
        Err(err) => {
            log::error!("Error {}", err);
            Err(err.into())
        }
    }
}

async fn execute_wol_command<'a>(
    _ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<String, Error> {
    let feature_param = get_param_value("mac_address", input);
    match feature_param {
        Some(found_feature_param) => match found_feature_param.parse::<MacAddress>() {
            Ok(address) => {
                let magic_packet = wake_on_lan::MagicPacket::new(&address.bytes());

                match magic_packet.send() {
                    Ok(_success) => {
                        log::debug!(
                            "Successfully send magic packet to host with mac address {}",
                            address
                        );
                        Ok("SEND".to_string())
                    }
                    Err(err) => {
                        log::error!(
                            "Could not send magic packet due to technical problems: {:?}",
                            err
                        );
                        Ok("ERROR".to_string())
                    }
                }
            }
            Err(err) => {
                log::error!(
                    "Given mac address {} is invalid. Cannot send magic packet for WoL {}",
                    found_feature_param,
                    err
                );
                Ok("ERROR".to_string())
            }
        },
        None => Ok("ERROR".to_string()),
    }
}




fn replace_list(
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
fn replace(input_string: String, ipaddress: &str, input: &ActionOrDataInput) -> (String, String) {
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

        let replacement_option = get_param_value(name.as_str(), input);

        if let Some(replacement) = replacement_option {
            result = result.replace(placeholder.as_str(), replacement.as_str());
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
        .map(|credential| (decrypt(credential, &input.crypto_key), credential.encrypted))
}

fn decrypt( credential: &Credential, crypto_key: &str ) -> String {
    if credential.encrypted {
        crypt::default_decrypt(&credential.value, crypto_key)
    }
    else {
        credential.value.clone()
    }
}

fn get_param_value(name: &str, input: &ActionOrDataInput) -> Option<String> {
    let value_from_feature = input.find_param(name).map(|param| param.value.clone());

    match value_from_feature {
        Some(value) => Some(value),
        None => {
            let default_value_from_plugin = input.find_default_param(name);

            default_value_from_plugin.map(|def| def.default_value.to_owned())
        }
    }
}

fn encode_base64(placeholder: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(Placeholder::Base64.strip_of_marker(placeholder))
}

fn find_plugin_for_feature<'a>(feature: &Feature, plugins: &'a [Plugin]) -> Option<&'a Plugin> {
    plugins.iter().find(|p| p.id == feature.id)
}

fn find_data_for_action_depency<'a>(depend: &DependsDef, plugin: &'a Plugin) -> Option<&'a Data> {
    plugin.data.iter().find(|d| d.id == depend.data_id)
}

fn response_data_match(dependency: &DependsDef, input: &str) -> Result<bool, Error> {
    let script = dependency.script.clone();
    let script_type = dependency.script_type.clone();

    let is_lua = matches!(script_type.as_str(), "lua");
    let is_rhai = matches!(script_type.as_str(), "rhai");

    if !is_lua && !is_rhai {
        return Err(Error::from(std::io::Error::new(
            ErrorKind::Other,
            "Only RHAI and LUA scripts are currently supported",
        )));
    }

    let mut result = false;

    if is_lua {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
            globals
                .set("input", "[[".to_string() + input + "]]")
                .expect("Could not set global value");

            if let Ok(value) = lua_ctx.load(&script).eval() {
                result = value;
            }
        });
    }
    else if is_rhai {
        let mut scope = Scope::new();
        
        scope.push("input", input.to_owned());

        let engine = Engine::new();        
        if let Ok(value) = engine.eval_with_scope::<bool>(&mut scope, &script) {
            result = value;
        }
    }

    Ok(result)
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
