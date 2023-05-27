use crate::models::{
    plugin::{action::Action, data::Data, Plugin},
    server::Feature,
};

use super::CommandArg;

pub mod replace;

pub fn data_args_to_command_args(data: &Data) -> Vec<CommandArg> {
    data.args
        .iter()
        .map(|a| CommandArg {
            name: a.arg_type.clone(),
            value: a.value.clone(),
        })
        .collect()
}

pub fn action_args_to_command_args(action: &Action) -> Vec<CommandArg> {
    action
        .args
        .iter()
        .map(|a| CommandArg {
            name: a.arg_type.clone(),
            value: a.value.clone(),
        })
        .collect()
}

pub fn plugin_default_params_to_command_args(plugin: &Plugin) -> Vec<CommandArg> {
    plugin
        .params
        .iter()
        .map(|p| CommandArg {
            name: p.name.clone(),
            value: p.default_value.clone(),
        })
        .collect()
}

pub fn feature_params_to_command_args(feature: &Feature) -> Vec<CommandArg> {
    feature
        .params
        .iter()
        .map(|p| CommandArg {
            name: p.name.clone(),
            value: p.value.clone(),
        })
        .collect()
}

pub fn action_params_to_command_args(action_params: Option<&str>) -> Vec<CommandArg> {
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
