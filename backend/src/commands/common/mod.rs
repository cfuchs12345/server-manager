use crate::models::{
    plugin::{common::ArgDef, ParamDef},
    server::Param,
};

use super::CommandArg;

pub mod replace;

pub fn args_to_command_args(vec: &[ArgDef]) -> Vec<CommandArg> {
    vec.iter()
        .map(|a| CommandArg {
            name: a.arg_type.clone(),
            value: a.value.clone(),
        })
        .collect()
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

pub fn string_params_to_command_args(action_params: Option<&str>) -> Vec<CommandArg> {
    let mut list = Vec::new();

    if let Some(action_params) = action_params {
        let split = action_params.split(',');

        for str in split {
            let single_param = str.split_at(str.find('=').unwrap());

            list.push(CommandArg {
                name: single_param.0.to_owned(),
                value: single_param.1[1..].to_owned(), // skip the first char which is still the separator
            });
        }
    }
    list
}
