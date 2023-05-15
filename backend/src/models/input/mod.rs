use super::{plugin::{common::ArgDef, ParamDef, action::Action, Plugin, data::Data}, server::{Param, Credential, Feature}};




#[derive (Debug, Default)]
pub struct ActionOrDataInput {
    pub command: String,
    args: Vec<ArgDef>,
    params: Vec<Param>,
    default_params: Vec<ParamDef>,
    action_params: Vec<Param>,
    credentials: Vec<Credential>,
    pub crypto_key: Option<String>
}
impl ActionOrDataInput {
    pub fn get_input_from_action(action: &Action,  action_params: Option<&str>, plugin: &Plugin, feature: &Feature, crypto_key: String ) -> ActionOrDataInput {
        let action_params = to_params(action_params);
        ActionOrDataInput{
            command: action.command.clone(),
            args: action.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            action_params,
            credentials: feature.credentials.clone(),
            crypto_key: Some(crypto_key)
        }
    }

    pub fn get_input_from_data(data: &Data, action_params: Option<&str>, plugin: &Plugin, feature: &Feature, crypto_key: &str) ->  ActionOrDataInput {
        let action_params = to_params(action_params);

        ActionOrDataInput{
            command: data.command.clone(),
            args: data.args.clone(),
            default_params: plugin.params.clone(),
            params: feature.params.clone(),
            action_params,
            credentials: feature.credentials.clone(),
            crypto_key: Some(crypto_key.to_owned())
        }
    }

    pub fn find_param(&self, param_name: &str) -> Option<Param> {
        let from_feature = self.params.iter().find( |param| param.name == param_name);

        if from_feature.is_some() {
            from_feature.cloned()
        }
        else { // there are also parameter coming with the request i.e. for sub actions. also check these...
            let from_request = self.find_action_param(param_name);

            if from_request.is_some() {
                from_request.cloned()
            }
            else {
                let default_value_from_plugin = self.find_default_param(param_name);

                default_value_from_plugin.map(|def| Param {
                    name: def.name.clone(),
                    value: def.default_value.clone()
                })
            }
        }
    }

    fn find_action_param(&self, param_name: &str) -> Option<&Param> {
        self.action_params.iter().find(|param| param.name == param_name)
    }

    pub fn find_default_param(&self, param_name: &str) -> Option<&ParamDef> {
        self.default_params.iter().find( |param| param.name == param_name)
    }

    pub fn find_credential(&self, credential_name: &str) -> Option<&Credential> {
        self.credentials.iter().find( |credential| credential.name == credential_name)
    }
    
    pub fn find_arg(&self, arg_type: &str) ->  Option<&ArgDef> {
        self.args.iter().find(|argdef| argdef.arg_type == arg_type)        
    }

    pub fn find_all_args(&self, arg_type: &str) ->  Vec<&ArgDef> {
        self.args.iter().filter(|argdef| argdef.arg_type == arg_type).collect()        
    }
}

fn to_params(action_params: Option<&str>) -> Vec<Param> {
    if action_params.is_none() {
        return Vec::new();
    }
    let mut list = Vec::new();

    let split = action_params.unwrap().split(',');

    for str in split {
        let single_param = str.split_at(str.find('=').unwrap());
        
        list.push(Param {
            name: single_param.0.to_owned(),
            value: single_param.1[1..].to_owned() // skip the first char which is still the separator
        });
    }

    list
}



