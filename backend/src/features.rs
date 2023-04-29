use std::io::ErrorKind;
use std::{time::Duration, borrow::Borrow};
use actix_web::Error;
use http::StatusCode;
use lazy_static::lazy_static;
use http::{response::Builder};
use reqwest::Response;
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use rlua::Lua;
use wake_on_lan;
use mac_address::MacAddress;
use crate::conversion;
use crate::plugin_types::{Plugin, Action, ArgDef, ParamDef, Data, DependsDef};
use crate::server_types::{Feature, Param, Credential, Server};

enum RegexType {
    ParamRegex,
    CredentialRegex,
    Base64Regex
}

#[derive (Debug)]
struct ActionOrDataInput {
    command: String,
    args: Vec<ArgDef>,
    params: Vec<Param>,
    default_params: Vec<ParamDef>,
    credentials: Vec<Credential>,
    accept_self_signed_ceritificates: bool
}
impl ActionOrDataInput {
    fn get_input_from_action(action: &Action, plugin: &Plugin, feature: &Feature, accept_self_signed_ceritificates: bool) -> ActionOrDataInput {
        ActionOrDataInput{
            command: action.command.clone(),
            args: action.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            credentials: feature.credentials.clone(),
            accept_self_signed_ceritificates
        }
    }

    fn get_input_from_data(data: &Data, plugin: &Plugin, feature: &Feature, accept_self_signed_ceritificates: bool) -> ActionOrDataInput {
        ActionOrDataInput{
            command: data.command.clone(),
            args: data.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            credentials: feature.credentials.clone(),
            accept_self_signed_ceritificates
        }
    }
}


lazy_static! {
    static ref PARAM_REGEX : Regex = Regex::new(
            RegexType::ParamRegex.get_pattern()
        ).unwrap();

    static ref CREDENTIAL_REGEX : Regex = Regex::new(
        RegexType::CredentialRegex.get_pattern()
    ).unwrap();

    static ref BASE64_REGEX : Regex = Regex::new(
        RegexType::Base64Regex.get_pattern()
    ).unwrap();   
}

impl RegexType {
	fn get_pattern(&self) -> &str {
		match self {
			RegexType::ParamRegex => r"(\$\{params\..*?\})",
			RegexType::CredentialRegex =>  r"(\$\{credentials\..*?\})",            
			RegexType::Base64Regex => r"(\$\{encode_base64\(.*?\)\})"
		}
	}

    pub fn extract_placeholders(&self, input: String) -> Vec<String> {       
        let matches = match self {
            RegexType::ParamRegex => PARAM_REGEX.find_iter(input.as_str()),
            RegexType::CredentialRegex => CREDENTIAL_REGEX.find_iter(input.as_str()),
            RegexType::Base64Regex => BASE64_REGEX.find_iter(input.as_str())
        };
    
        matches.map(|mat| mat.as_str().to_owned()).collect()
    } 

    pub fn strip_of_marker(&self, value: &String) -> String {
        match self {
			RegexType::ParamRegex => value.replace("${params.", "").replace("}", ""),
			RegexType::CredentialRegex =>  value.replace("${credentials.", "").replace("}", ""),
			RegexType::Base64Regex => value.replace("${encode_base64(", "").replace(")}", ""),
		}
    }
}



pub async fn execute_action( server: &Server, feature: &Feature, plugin: &Plugin, action_id: &str, accept_self_signed_certificates: bool) -> Result<bool, Error> {
    match plugin.actions.iter().find( |plugin| plugin.id == action_id) {
        Some(plugin_action) => {
            let input: ActionOrDataInput = ActionOrDataInput::get_input_from_action(plugin_action, plugin, feature, accept_self_signed_certificates);

            execute_command(server.ipaddress.clone(), &input).await.map(|_| true)
        },
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, plugin.id);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}


pub async fn execute_data_query( server: &Server, plugins: &Vec<Plugin>, accept_self_signed_certificates: bool, template_engine: &handlebars::Handlebars<'static>) -> Result<Vec<String>, Error> {
    let mut results: Vec<String> = vec![];

    let tuples: Vec<(&Feature, &Plugin)> = server.features.iter().filter_map( |f| {
        Some( (f, find_plugin_for_feature(&f, &plugins)?) )
    }).collect();
    
    for tuple in tuples {
        for data in &tuple.1.data {
            let res = execute_specific_data_query( server, tuple.1, tuple.0, &data, accept_self_signed_certificates ).await?;

            if !data.template.is_empty() {
                let result = conversion::convert_json_to_html(data.template.as_str(), res, template_engine, &data)?;
                results.push(result);
            }                   
            else {
                results.push(res);
            }

        }
    }          

    Ok(results)  
}

pub async fn execute_specific_data_query( server: &Server, plugin: &Plugin, feature: &Feature, data: &Data, accept_self_signed_certificates: bool) -> Result<String, Error> {
    let input: ActionOrDataInput = ActionOrDataInput::get_input_from_data(data, plugin, feature, accept_self_signed_certificates);

    execute_command(server.ipaddress.clone(), &input).await
}

pub async fn check_condition_for_action_met(server: &Server, plugin: &Plugin, feature: &Feature, action_id: &str, accept_self_signed_certificates: bool) -> Result<bool, Error> {
    match plugin.actions.iter().find(|a| a.id == action_id) {
        Some(action) => {
            let mut res = true;

            for depends in &action.depends {
                match find_data_for_action_depency(&depends, plugin) {
                    Some(data) => {
                        let response =  execute_specific_data_query( server, plugin, feature, data, accept_self_signed_certificates).await?;

                        res &= data_match(depends, response.as_str() )?;

                        if !res {
                            log::info!("Depencies for data {} of plugin {} for server {} not met .Reasponse was {}", data.id, plugin.id, server.ipaddress, response);
                            break;
                        }
                    },
                    None => {
                        let error = format!("dependent data with id  {} not found for action {}", depends.data_id,  action_id);
                        log::error!("{}", error);
                        res = false;
                        break;
                    }
                }
            }
            Ok(res)
        },
        None => {
            let error = format!("{} is not a action of plugin {}", action_id, plugin.id);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}



async fn execute_command(ipaddress: String, input: &ActionOrDataInput) ->  Result<String, Error> {
    match input.command.as_str() {
        "http" => {
            execute_http_command(ipaddress, input).await
        },
        "wol" => {
            execute_wol_command(ipaddress, input).await
        }
        y => {
            let error = format!("Action command {} is not implemented ", y);
            log::error!("{}", error);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, error)))
        }
    }
}


async fn execute_http_command(ipaddress: String, input: &ActionOrDataInput) -> Result<String, Error> {
    let body = find_arg(&input.args, "body");
    let url = find_arg(&input.args, "url");
    let method = find_arg(&input.args, "method");
    let headers = find_all_args(&input.args, "header" );
    
    let normal_and_masked_url: (String,String) = replace(&url, &ipaddress, &input);
    let normal_and_masked_body: (String,String) = replace(&body, &ipaddress, &input);
    let normal_and_replaced_headers: Vec<(String, String)> = replace_list(headers, &ipaddress, &input);

    if body.len() > 0 {
        log::debug!("About to execute method {} on url {} with body {}", method, normal_and_masked_url.1, normal_and_masked_body.1);

        log::info!("About to execute method {} on url {} with body {}", method, normal_and_masked_url.0, normal_and_masked_body.0);
    }
    else {
        log::debug!("About to execute method {} on url {}", method, normal_and_masked_url.1);

        log::debug!("About to execute method {} on url {}", method, normal_and_masked_url.0);
    }
    

    match execute_http_request(normal_and_masked_url.0, method, normal_and_replaced_headers, normal_and_masked_body.0, &input.accept_self_signed_ceritificates ).await {
        Ok(response) => {
            let text = response.text().await.unwrap_or_default();
            log::debug!("Response for http request to url {} was: {}", normal_and_masked_url.1, text);
            Ok(text)
        }
        Err(err) => {
            log::error!("Error {}", err);
            Err(Error::from(std::io::Error::new(ErrorKind::Other, err)))
        }
    }
}

async fn execute_wol_command(_ipaddress: String, input: &ActionOrDataInput) ->  Result<String, Error> {
    let feature_param = get_param_value("mac_address", input);
    match feature_param {
        Some(found_feature_param) => {
            match found_feature_param.parse::<MacAddress>() {
                Ok(address) => {
                    let magic_packet = wake_on_lan::MagicPacket::new(&address.bytes());

                    match magic_packet.send()  {
                        Ok(_success) => {
                            log::debug!("Successfully send magic packet to host with mac address {}", address);
                            Ok("SEND".to_string())
                        },
                        Err(err) => {
                            log::error!("Could not send magic packet due to technical problems: {:?}", err);
                            Ok("ERROR".to_string())
                        }

                    }         
                },
                Err(err) => {
                    log::error!("Given mac address {} is invalid. Cannot send magic packet for WoL {}", found_feature_param, err);
                    Ok("ERROR".to_string())
                }
            }
        },
        None => {
            Ok("ERROR".to_string())
        }
    }
}

async fn execute_http_request(url: String, method: String, headers: Vec<(String, String)>, body: String, accept_self_signed_certificates: &bool ) -> Result<Response, reqwest::Error>  {
    let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(accept_self_signed_certificates.clone())
    .timeout(Duration::from_secs(1))
    .build()
    .unwrap();

    let header_map: http::HeaderMap = headers_to_map(headers);

    let result = match method.as_str() {
        "get" => client.get(url).headers(header_map).send().await,
        "post" => client.post(url).headers(header_map).body(body).send().await,
        y => {
            let response = Builder::new()
            .status(StatusCode::PRECONDITION_FAILED)
            .body(format!("method {} not supported", y))
            .unwrap();
            Ok(Response::from(response))
        }
    };

    result
}

fn headers_to_map(headers: Vec<(String, String)>) -> http::HeaderMap {
    let mut header_map: http::HeaderMap = http::HeaderMap::new();

    for header in headers {
        let res = header.0.split_once("=");

        if res.is_none() {
            log::error!("Header {} is invalid. Container no equals sign (=)", header.1);
            continue;
        }
        let split = res.unwrap();

        let name = http::header::HeaderName::from_lowercase(split.0.to_lowercase().as_bytes());
        let value = http::header::HeaderValue::from_str(split.1);

        if name.is_err() || value.is_err() {
            log::error!("Header {} is invalid", header.1);
        }
        else {
            header_map.insert(name.unwrap(), value.unwrap());
        }
    }

    header_map
}

fn replace_list ( input_strings: Vec<String>, ipaddress: &str,  input: &ActionOrDataInput) -> Vec<(String, String)> {
    let mut result:Vec<(String, String)> = vec![];

    for input_string in input_strings {
        let res = replace(&input_string, ipaddress, input);
        result.push(res);
    }

    result
}
fn replace ( input_string: &String, ipaddress: &str,  input: &ActionOrDataInput) -> (String, String) {
    let mut result : String = input_string.to_owned().clone();
    let mut masked: String;

    result = result.replace( "${IP}", ipaddress);
    result = replace_param(result, input);
    let both:(String,String) = replace_credentials(result, input); // we now have two string - the unmasked and the masked which can be logged for example
    result = both.0;
    masked = both.1;
    result = replace_base64_encoded(result); // base 64 encode should happen on both idependently
    masked = replace_base64_encoded(masked); // actually the base 64 encoded masked version outputs an incorrect encoded value

   (result.to_owned(), masked.to_owned())
}

fn replace_param(input_string: String, input: &ActionOrDataInput) -> String {
    let mut result = input_string.clone();

    for placeholder in RegexType::ParamRegex.extract_placeholders(input_string) {
        let name = RegexType::ParamRegex.strip_of_marker(&placeholder);

        let replacement = get_param_value(name.as_str(), input);

        if replacement.is_some() {
            result = result.replace(placeholder.as_str(), replacement.unwrap().as_str());
        }
        else {
            log::error!("Found no replacement for placeholder {}", placeholder);
        }
    }
    result
}

fn replace_credentials(input_string: String, input: &ActionOrDataInput) -> (String, String) {
    let mut result = input_string.clone();
    let mut masked = input_string.clone();

    for placeholder in RegexType::CredentialRegex.extract_placeholders(input_string) {
        let name = RegexType::CredentialRegex.strip_of_marker(&placeholder);

        let replacement = get_credential_value(name.as_str(), &input);
        
        if replacement.is_some() {
            
            let replacement_tuple = replacement.unwrap();
            
            result = result.replace(placeholder.as_str(), replacement_tuple.0.as_str());
            if replacement_tuple.1 {
                masked = masked.replace(placeholder.as_str(), "******");
            }
            else {
                masked = masked.replace(placeholder.as_str(), replacement_tuple.0.as_str());
            }
        }
        else {
            log::error!("Found no replacement for placeholder {}", placeholder);
        }
    }
    (result, masked)
}

fn replace_base64_encoded(input: String) -> String {
    let mut result = input.clone();

    for placeholder in RegexType::Base64Regex.extract_placeholders(input) {
        let to_encode = RegexType::Base64Regex.strip_of_marker(&placeholder);

        let replacement = encode_base64(&to_encode);

        result = result.replace(placeholder.as_str(), replacement.as_str());
    }    

    result
}

fn get_credential_value(name: &str, input: &ActionOrDataInput ) -> Option<(String, bool)> {
    let from_feature: Vec<(String, bool)> = input.credentials.iter().filter( |credential| credential.name == name).map(|credential| (credential.value.clone(), credential.encrypted )).collect();

    let value: Option<(String, bool)> = match from_feature.iter().next() {
        Some(value) => Some( (value.0.to_owned(), value.1) ),
        None => None
    };

    value
}


fn get_param_value(name: &str, input: &ActionOrDataInput ) -> Option<String> {
    let from_feature: Vec<String> = input.params.iter().filter( |param| param.name == name).map(|param| param.value.clone()).collect();

    let value: Option<String> = match from_feature.iter().next() {
        Some(value) => Some(value.to_owned()),
        None => {
            let from_plugin: Vec<String> = input.default_params.iter().filter( |default_param| default_param.name == name).map(|default_param| default_param.default_value.clone()).collect();

            match from_plugin.iter().next() {
                Some(value) => Some(value.to_owned()),
                None => None
            }
        }
    };

    value
}

fn encode_base64(placeholder: &String) -> String {
    general_purpose::STANDARD_NO_PAD.encode( RegexType::Base64Regex.strip_of_marker(placeholder) )
}



fn find_arg(args: &Vec<ArgDef>, arg_type: &str) -> String {
    let matched_args = find_all_args(args, arg_type);

    
    matched_args.iter().next().unwrap_or("".to_string().borrow()).to_string()
}

fn find_all_args(args: &Vec<ArgDef>, arg_type: &str) -> Vec<String> {
    let matched_args = args.iter().filter( |arg_def| arg_def.arg_type == arg_type).map( |arg_def| arg_def.value.clone()).collect();
    
    matched_args
}

fn find_plugin_for_feature<'a> ( feature: &Feature, plugins: &'a Vec<Plugin> ) -> Option<&'a Plugin> {
    plugins.iter().find( |p| p.id == feature.id)
}

fn find_data_for_action_depency<'a> ( depend: &DependsDef, plugin: &'a Plugin) -> Option<&'a Data> {
    plugin.data.iter().find( |d| d.id == depend.data_id)
}

fn data_match(dependency: &DependsDef, input: &str) -> Result<bool, Error> {
    let script = dependency.script.clone();
    let script_type = dependency.script_type.clone();

    let is_lua = match script_type.as_str() {
        "lua" => true,
        _ => false,
    };

    if !is_lua {
        return Err(Error::from(std::io::Error::new(ErrorKind::Other, "Only LUA scripts are currently supported")));
    }

    let mut result = false;
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals
            .set("input", "[[".to_string() + input + "]]")
            .expect("Could not set global value");

        result = lua_ctx.load(&script).eval().unwrap();
    });

    Ok(result)
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_param_extract() {
        assert_eq!(  RegexType::ParamRegex.extract_placeholders("test ${params.test}".to_string()).len(), 1);
        assert_ne!(  RegexType::ParamRegex.extract_placeholders( "test params.test".to_string()).len(), 1);
        assert_eq!( RegexType::ParamRegex.extract_placeholders("${params.protocol}://${credentials.username}:${credentials.password}192.168.178.20:${params.port}/${params.command}".to_string()).len(), 3);


        assert_eq!(  RegexType::Base64Regex.extract_placeholders("${encode_base64(USERNAME)}".to_string()).first().unwrap().to_owned(), "${encode_base64(USERNAME)}".to_owned());
    }

    #[test]
    fn test_regex_strip_of_marker() {
        assert_eq!( RegexType::ParamRegex.strip_of_marker(&"${params.test}".to_string()), "test");
        assert_eq!( RegexType::Base64Regex.strip_of_marker(&"${encode_base64(USERNAME)}".to_string()), "USERNAME");
    }

    #[test]
    fn test_encode_bas64() {
        assert_eq!(encode_base64(&"USERNAME".to_string()), "VVNFUk5BTUU");

        assert_eq!(encode_base64(&"test:test".to_string()), "dGVzdDp0ZXN0");
        assert_eq!(encode_base64(&"cfPkXYyADGWAimUFtG6g6YTYF7sckzOowz2vpQb4nR4Rv5Wyc8UzSYiEX3JjRaMsMG8GuipVd5G3W7zQ:fKreW9R548bI0918LWTQPDLCDWCjszlhpXNgLORPFaPeVK+RZ6HQn9o2znTLCuFijLKLHxvtkdtx5X32".to_string()), "dGVzdDp0ZXN0");
    }
}
