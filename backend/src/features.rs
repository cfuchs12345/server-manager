use crate::plugin_types::{Action, Data, DependsDef, Plugin, SubAction};
use crate::server_types::{Credential, Feature, Server};
use crate::types::{ActionOrDataInput, ConditionCheckResult, DataResult};
use crate::{conversion, crypt, http_functions, inmemory};
use actix_web::Error;
use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use mac_address::MacAddress;
use regex::Regex;
use std::io::ErrorKind;

use rhai::{Engine, Scope};
use rlua::Lua;

/// This enum hides the actual regular expressions and the matching and provides methods for
/// * easy extraction of matched strings
/// * strip of the markers and returning the actual name of the placeholder
enum Placeholder {
    Param,
    Credential,
    Base64,
}

#[derive(PartialEq, Eq)]
enum CheckType {
    OnlyMainFeatures,
    OnlySubFeatures
}


lazy_static! {
    static ref PARAM_REGEX: Regex = Regex::new(Placeholder::Param.get_pattern()).unwrap();
    static ref CREDENTIAL_REGEX: Regex = Regex::new(Placeholder::Credential.get_pattern()).unwrap();
    static ref BASE64_REGEX: Regex = Regex::new(Placeholder::Base64.get_pattern()).unwrap();
    static ref TEMPLATE_SUB_ACTION_REGEX: Regex = Regex::new(r"(\[\[Action .*?\]\])").unwrap();
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
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `action_id`- the identifier of the action
/// * `action_params`- optional params for an action
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_action(
    server: &Server,
    feature: &Feature,
    action_id: &str,
    action_params: Option<&str>,
    crypto_key: String,
) -> Result<bool, Error> {
    let plugin_res = inmemory::get_plugin(feature.id.as_str());
    if plugin_res.is_none() {
        log::error!("Plugin not found {}", feature.id);
        return Ok(false);
    }

    let plugin = plugin_res.unwrap();

    match plugin.find_action(action_id) {
        Some(plugin_action) => {
            let input: ActionOrDataInput = ActionOrDataInput::get_input_from_action(
                plugin_action,
                action_params,
                &plugin,
                feature,
                crypto_key,
            );

            execute_command(server.ipaddress.clone(), &input)
                .await
                .map(|res| {
                    log::info!("Response for server action was: {:?}", res);
                    true
                })
        }
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, feature.id);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}

/// Executes all data queries on the given server for all given plugins
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `template_engine` - the template engine struct that is used to render the output in a readable format
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_data_query(
    server: &Server,
    template_engine: &handlebars::Handlebars<'static>,
    crypto_key: &str,
) -> Result<Vec<DataResult>, Error> {
    let mut results: Vec<DataResult> = vec![];

    for feature in &server.features {
        let plugin_opt = inmemory::get_plugin(feature.id.as_str());

        if plugin_opt.is_none() {
            log::error!("plugin {} not found", feature.id);
            continue;
        }
        let plugin = plugin_opt.unwrap();

        for data in &plugin.data {
            log::debug!("Plugin data execute {} {}", plugin.id, data.id);

            if !data.output {
                log::debug!("Skipping data entry {} of plugin {} since it is marked as output = false", data.id, plugin.id);
                continue;
            }
            let data_response =
                execute_specific_data_query(server, &plugin, feature, data, None, crypto_key)
                    .await?;



            match data_response {
                Some(response) => {
                    let result = if !data.template.is_empty() {
                        // convert the output with the template
                        conversion::convert_json_to_html(
                            data.template.as_str(),
                            response,
                            template_engine,
                            data,
                        )?

                    } else {
                        // no template - just append
                        response
                    };

                    let enriched_result = inject_meta_data_for_actions(result, feature, data);

                    let actions = extract_actions(&enriched_result);
                    
                    let check_results = check_action_conditions(server, actions, crypto_key).await;
                    log::info!("-- {:?}", check_results);
                    results.push(DataResult{
                        ipaddress: server.ipaddress.clone(),
                        result: enriched_result,
                        check_results
                    });
                },
                None => {

                }
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
/// * `persistence` - the persistence struct that helps to interact with the underlying database
async fn execute_specific_data_query(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    data: &Data,
    action_params: Option<&str>,
    crypto_key: &str,
) -> Result<Option<String>, Error> {
    let input: ActionOrDataInput =
        ActionOrDataInput::get_input_from_data(data, action_params, plugin, feature, crypto_key);

    execute_command(server.ipaddress.clone(), &input).await
}


/// Checks if all conditions that are defined for an action of a plugin are met and that it can be executed by the user
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `actiond`- the plugin action to check
/// * `persistence` - the persistence struct that helps to interact with the underlying database

pub async fn check_condition_for_action_met(
    server: &Server,
    feature: Option<&Feature>,
    action: Option<&Action>,
    action_params: Option<String>,
    crypto_key: &str,
) -> ConditionCheckResult {
    if feature.is_none() || action.is_none()  {
        return ConditionCheckResult {
            ipaddress: server.ipaddress.clone(),
            result: false,
            ..Default::default()
        };
    }

    let plugin_res = inmemory::get_plugin(feature.unwrap().id.as_str());

    if plugin_res.is_none()  {
        return ConditionCheckResult {
            action_id: action.unwrap().id.clone(),
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id.clone(),
            ipaddress: server.ipaddress.clone(),
            result: false,
        };
    }

    let plugin = plugin_res.unwrap();

    let status: Vec<crate::types::Status> =
        crate::status::status_check(vec![server.ipaddress.clone()], true)
            .await
            .unwrap_or(Vec::new());

    let mut result = match action.unwrap().available_for_state {
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

    if !result {
        // check if status dependency already failed - early exit
        return ConditionCheckResult {
            action_id: action.unwrap().id.clone(),
            action_params: action_params.unwrap_or_default(),
            feature_id: feature.unwrap().id.clone(),
            ipaddress: server.ipaddress.clone(),
            result: false,
        };
    }

    if let Some(status) = status.first() {
        if status.is_running {
            // if not running, no need to start any request
            // now check data dependencies one by one
            for depends in &action.unwrap().depends {
                match find_data_for_action_depency(depends, &plugin) {
                    Some(data) => {
                        let response = execute_specific_data_query(
                            server,
                            &plugin,
                            &feature.unwrap(),
                            data,
                            action_params.as_deref(),
                            crypto_key,
                        )
                        .await
                        .unwrap_or_default();

                        result &=
                            response_data_match(depends, response.clone()).unwrap_or_default();
                        if !result {
                            log::debug!("Depencies for data {} of plugin {} for server {} not met .Reasponse was {:?}", data.id, feature.unwrap().id, server.ipaddress, response);
                            break;
                        }
                    }
                    None => {
                        let error = format!(
                            "dependent data with id  {} not found for action {}",
                            depends.data_id, action.unwrap().id
                        );
                        log::error!("{}", error);
                        result = false;
                        break;
                    }
                }
            }
        } else if !action.unwrap().depends.is_empty() {
            result = false;
        }
    };

    ConditionCheckResult {
        action_id: action.unwrap().id.clone(),
        action_params: action_params.unwrap_or_default(),
        feature_id: feature.unwrap().id.clone(),
        ipaddress: server.ipaddress.clone(),
        result,
    }
}


fn inject_meta_data_for_actions(input : String, feature: &Feature, data: &Data) -> String {    
    input.replace("[[Action ", format!("[[Action feature.id=\"{}\" data.id=\"{}\" ", feature.id, data.id).as_str())
}

fn extract_actions(input: &str) -> Vec<SubAction> {
    let mut result = Vec::new();
    let groups: Vec<String> = TEMPLATE_SUB_ACTION_REGEX.find_iter(input).map(|mat| mat.as_str().to_owned()).collect();
    for group in groups {
        result.push(SubAction::from(group));
    }
    
    result
}

async fn check_action_conditions(server: &Server, sub_actions: Vec<SubAction>, crypto_key: &str) -> Vec<ConditionCheckResult>{
    let mut results = Vec::new();

    for sub_action in &sub_actions {
        if sub_action.feature_id.is_none() || sub_action.action_id.is_none() {
            continue;
        }
        let feature = server.find_feature(sub_action.feature_id.as_ref().unwrap().to_owned());
        let plugin = inmemory::get_plugin(sub_action.feature_id.as_ref().unwrap().as_str());
        
        if plugin.is_none() {
            continue;
        }

        let res = check_condition_for_action_met(server, feature,  plugin.unwrap().find_action(sub_action.action_id.as_ref().unwrap().as_str()), sub_action.action_params.clone(), crypto_key.clone()).await;
        results.push(res);
    }
    results
}

pub async fn check_main_action_conditions() {
    let servers = inmemory::get_all_servers();
    let crypto_key = inmemory::get_crypto_key();

    let mut vec: Vec<ConditionCheckResult> = Vec::new();
    for server in servers {
        let mut res = check_server_feature_conditions(server, &crypto_key, CheckType::OnlyMainFeatures).await;
        vec.append(&mut res);
    }
    
    for result in vec {
        inmemory::add_condition_result(result);
    }
}

async fn check_server_feature_conditions(server: Server, crypto_key: &str, check_type: CheckType) -> Vec<ConditionCheckResult> {
    let mut vec = Vec::new();

    for feature in server.clone().features {
        let plugin_res = inmemory::get_plugin(feature.id.as_str());
        if plugin_res.is_none() {
            log::error!("plugin with id {} not found", feature.id);
            continue;
        }
        if let Some(plugin) = plugin_res {
            for action in plugin.clone().actions {
                if check_type == CheckType::OnlyMainFeatures && !action.show_on_main {
                    log::debug!("Skipping action condition check for non-main action {} of feature {}", action.id, feature.id);
                    continue;
                }
                else if check_type == CheckType::OnlySubFeatures && action.show_on_main {
                    log::debug!("Skipping action condition check for sub-main action {} of feature {}", action.id, feature.id);
                    continue;
                }
                let server_clone = server.clone();
                let feature_clone = feature.clone();
                let action_clone = action.clone();

                let check_res = check_condition_for_action_met(
                    &server_clone,
                    Some(&feature_clone),
                    Some(&action_clone),
                    None,
                    crypto_key,
                )
                .await;

                vec.push(check_res);
            }
        }
    }

    vec
}

async fn execute_command<'a>(
    ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<Option<String>, Error> {
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
) -> Result<Option<String>, Error> {
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

    if normal_and_masked_url.0.trim().is_empty() {
        log::warn!(
            "Given url is empty after replacing placeholders. Was before replace: {}. Request will not be executed",
            url.value
        );
        return Ok(None);
    }

    let text = http_functions::execute_http_request(
        normal_and_masked_url.0,
        method.value.as_str(),
        Some(normal_and_replaced_headers),
        Some(normal_and_masked_body.0),
    )
    .await
    .unwrap_or_default();

    Ok(Some(text))
}

async fn execute_wol_command<'a>(
    _ipaddress: String,
    input: &ActionOrDataInput,
) -> Result<Option<String>, Error> {
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
                        Ok(Some("SEND".to_string()))
                    }
                    Err(err) => {
                        log::error!(
                            "Could not send magic packet due to technical problems: {:?}",
                            err
                        );
                        Err(Error::from(err))
                    }
                }
            }
            Err(err) => {
                log::error!(
                    "Given mac address {} is invalid. Cannot send magic packet for WoL {}",
                    found_feature_param,
                    err
                );
                Err(Error::from(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    err,
                )))
            }
        },
        None => Ok(None),
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

fn decrypt(credential: &Credential, crypto_key: &str) -> String {
    if credential.encrypted {
        crypt::default_decrypt(&credential.value, crypto_key)
    } else {
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

fn find_data_for_action_depency<'a>(depend: &DependsDef, plugin: &'a Plugin) -> Option<&'a Data> {
    plugin.data.iter().find(|d| d.id == depend.data_id)
}

fn response_data_match(dependency: &DependsDef, input: Option<String>) -> Result<bool, Error> {
    if input.is_none() {
        return Ok(false);
    }
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
                .set("input", "[[".to_string() + input.unwrap().as_str() + "]]")
                .expect("Could not set global value");

            if let Ok(value) = lua_ctx.load(&script).eval() {
                result = value;
            }
        });
    } else if is_rhai {
        let mut scope = Scope::new();

        scope.push("input", input.unwrap());

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
