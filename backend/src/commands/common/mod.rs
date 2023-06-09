use jsonpath_rust::JsonPathQuery;

use crate::{
    common::{self},
    models::{
        error::AppError,
        plugin::{
            common::{ArgDef, ArgType},
            ParamDef, Plugin,
        },
        server::{Param, Server},
    },
    plugin_execution,
};

use super::CommandArg;

pub mod replace;

pub async fn args_to_command_args(
    vec: &[ArgDef],
    server: &Server,
    plugin: &Plugin,
    crypto_key: &str,
    silent: &bool,
) -> Result<Vec<Vec<CommandArg>>, AppError> {
    let list_from_data_args = vec.iter().find(|a| a.arg_type == ArgType::ListFromData);
    let normal_args = vec.iter().filter(|a| a.arg_type != ArgType::ListFromData);

    let mut vec_outer = Vec::new();

    let non_list_command_args = normal_args
        .map(|a| CommandArg {
            name: a.name.clone(),
            value: a.value.clone(),
        })
        .collect();

    if list_from_data_args.is_none() {
        vec_outer.push(non_list_command_args);

        return Ok(vec_outer);
    }

    if let Some(arg) = list_from_data_args {
        let value = &arg.value;

        if let Some(feature) = server.find_feature(&plugin.id) {
            if let Some(source_data_id) = &arg.data_id {
                if let Some(data) = plugin.data.iter().find(|d| *d.id == *source_data_id) {
                    let responses = plugin_execution::execute_specific_data_query(
                        server, plugin, &feature, data, None, crypto_key, silent,
                    )
                    .await?;

                    for response in responses {
                        let json = serde_json::from_str::<serde_json::Value>(response.1.as_str())?;

                        if value.starts_with('$') {
                            if let Ok(extracted_values) = json.clone().path(value) {
                                let list: Vec<String> = if extracted_values.is_array() {
                                    extracted_values
                                        .as_array()
                                        .unwrap_or(&Vec::new())
                                        .iter()
                                        .flat_map(common::convert_value_to_str)
                                        .collect()
                                } else if let Some(str) =
                                    common::convert_value_to_str(&extracted_values)
                                {
                                    vec![str]
                                } else {
                                    Vec::new()
                                };

                                let list_arg_data_values: Vec<CommandArg> = list
                                    .iter()
                                    .map(|s| CommandArg {
                                        name: arg.name.clone(),
                                        value: s.to_owned(),
                                    })
                                    .collect();

                                for arg in list_arg_data_values {
                                    let mut inner_list_clone = non_list_command_args.clone();
                                    inner_list_clone.push(arg);

                                    vec_outer.push(inner_list_clone);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(vec_outer)
}

pub fn param_def_to_command_args(vec: &[ParamDef]) -> Vec<CommandArg> {
    vec.iter()
        .map(|p| CommandArg {
            name: p.name.clone(),
            value: p.default_value.clone(),
        })
        .collect()
}

pub fn params_to_command_args(vec: &[Param]) -> Vec<CommandArg> {
    vec.iter()
        .map(|p| CommandArg {
            name: p.name.clone(),
            value: p.value.clone(),
        })
        .collect()
}

pub fn string_params_to_command_args(
    action_params_opt: Option<String>,
) -> Result<Vec<CommandArg>, AppError> {
    let mut list = Vec::new();

    if let Some(action_params) = &action_params_opt {
        let split = action_params.split(',');

        for str in split {
            let single_param = str.split_at(str.find('=').ok_or(AppError::InvalidArgument(
                "Param".to_owned(),
                action_params_opt.clone(),
            ))?);

            list.push(CommandArg {
                name: single_param.0.to_owned(),
                value: single_param.1[1..].to_owned(), // skip the first char which is still the separator
            });
        }
    }
    Ok(list)
}
