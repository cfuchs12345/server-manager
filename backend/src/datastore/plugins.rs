use futures::future::join_all;
use futures::lock::Mutex;
use lazy_static::lazy_static;
use log::debug;

use std::collections::HashMap;
use std::{fs::File, io::BufReader};

use crate::event_handling::{ListSource, ObjectType};
use crate::models::error::AppError;
use crate::models::plugin::Plugin;
use crate::{datastore, event_handling};

use super::persistence;
use super::Entry;

const TABLE_PLUGIN_CONFIG: &str = "plugin_config";

lazy_static! {
    static ref PLUGIN_NAME_TO_FILENAME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn get_all_plugin_filenames(plugin_base_path: &str) -> Result<Vec<String>, AppError> {
    let mut plugin_file_names: Vec<String> = vec![];

    let paths = std::fs::read_dir(plugin_base_path)?;

    for path in paths {
        let os_string = path?.file_name();

        if let Some(file_name) = os_string.to_str() {
            if file_name.ends_with(".json") {
                plugin_file_names.push(file_name.to_string());
            }
        }
    }
    Ok(plugin_file_names)
}

pub fn init_cache_silent() {
    match init_cache() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error during plugin cache init: {}", err);
        }
    }
}

pub fn init_cache() -> Result<usize, AppError> {
    let plugins = futures::executor::block_on(load_all())?;

    debug!("Plugins loaded: {:?}", plugins);

    datastore::cache_plugins(plugins)
}

async fn load_all() -> Result<Vec<Plugin>, AppError> {
    datastore::clean_plugin_cache()?;
    let plugin_base_path = datastore::get_config()?
        .get_string("plugin_base_path")
        .map_err(|err| AppError::Unknown(format!("{}", err)))?;

    let plugin_file_names = get_all_plugin_filenames(plugin_base_path.as_str())?;

    let plugins: Vec<Option<Plugin>> =
        join_all(plugin_file_names.iter().map(|plugin_file_name| async {
            match load_plugin(plugin_base_path.as_str(), plugin_file_name).await {
                Ok(plugin) => {
                    PLUGIN_NAME_TO_FILENAME
                        .lock()
                        .await
                        .insert(plugin.id.clone(), plugin_file_name.to_owned());

                    Some(plugin)
                }
                Err(err) => {
                    log::error!(
                        "Could not load plugin from file {}. Error was: {}",
                        plugin_file_name.clone(),
                        err
                    );
                    None
                }
            }
        }))
        .await;

    Ok(plugins.iter().flat_map(|p| p.to_owned()).collect())
}

pub async fn load_plugin(
    plugin_base_path: &str,
    plugin_file_name: &str,
) -> Result<Plugin, AppError> {
    match File::open(plugin_base_path.to_owned() + "/" + plugin_file_name) {
        Ok(file) => {
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            match serde_json::from_reader(reader) {
                Ok::<Plugin, _>(plugin) => {
                    log::debug!("plugin loaded: {:?}", plugin);
                    Ok(plugin)
                }
                Err(err) => {
                    log::error!(
                        "Error while parsing plugin file {} was: {}",
                        plugin_file_name,
                        err
                    );
                    Err(AppError::from(err))
                }
            }
        }
        Err(err) => Err(AppError::from(err)),
    }
}

pub async fn get_disabled_plugins() -> Result<Vec<String>, AppError> {
    match persistence::get(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(res) => match res {
            Some(entry) => Ok(entry.value.split(',').map(|e| e.to_string()).collect()),
            None => Ok(Vec::new()),
        },
        Err(err) => Err(err),
    }
}

pub async fn disable_plugins(plugin_ids: Vec<String>) -> Result<bool, AppError> {
    let existing = match persistence::get(TABLE_PLUGIN_CONFIG, "disabled_ids").await? {
        Some(entry) => entry.value.split(',').map(|e| e.to_string()).collect(),
        None => Vec::new(),
    };

    match persistence::delete(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(_res) => {
            match persistence::insert(
                TABLE_PLUGIN_CONFIG,
                Entry {
                    key: "disabled_ids".to_string(),
                    value: plugin_ids.join(",").to_string(),
                },
            )
            .await
            {
                Ok(_res) => {
                    event_handling::handle_list_change(
                        ListSource::new(ObjectType::DisabledPlugins, plugin_ids),
                        ListSource::new(ObjectType::DisabledPlugins, existing),
                    )?;
                    Ok(true)
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn is_plugin_disabled(plugin_id: &str) -> Result<bool, AppError> {
    match persistence::get(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(res) => {
            match res {
                Some(entry) => {
                    let mut ids = entry.value.split(',');
                    Ok(ids.any(|id| *id == *plugin_id))
                }
                None => Ok(false), // default is that it is activated
            }
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plugin::{
        action::{ActionDef, State},
        common::{ArgDef, ArgType, Script},
        detection::{DetectionDef, DetectionEntry},
        ParamDef,
    };
    use config::Config;

    #[test]
    fn test_serialize_plugin() {
        let testee: Plugin = Plugin {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            server_icon: "".to_string(),
            detection: DetectionDef {
                list: vec![DetectionEntry {
                    params: vec![ParamDef {
                        name: "port".to_string(),
                        default_value: "80".to_string(),
                        param_type: "string".to_string(),
                        mandatory: true,
                    }],
                    args: vec![
                        ArgDef {
                            name: "method".to_string(),
                            arg_type: ArgType::String,
                            value: "get".to_string(),
                            data_id: None,
                        },
                        ArgDef {
                            name: "url".to_string(),
                            arg_type: ArgType::String,
                            value: "url".to_string(),
                            data_id: None,
                        },
                    ],
                }],
                script: Script {
                    script_type: "lua".to_string(),
                    script: "Dummy script".to_string(),
                },
                detection_possible: false,
                command: "http".to_string(),
            },
            credentials: vec![],
            params: vec![],
            data: vec![],
            notifications: vec![],
            actions: vec![ActionDef {
                id: "".to_string(),
                name: "".to_string(),
                show_on_main: false,
                depends: vec![],
                available_for_state: State::Any,
                needs_confirmation: false,
                description: "".to_string(),
                icon: "".to_string(),
                command: "http".to_string(),
                args: vec![
                    ArgDef {
                        name: "method".to_string(),
                        arg_type: ArgType::String,
                        value: "get".to_string(),
                        data_id: None,
                    },
                    ArgDef {
                        name: "url".to_string(),
                        arg_type: ArgType::String,
                        value: "url".to_string(),
                        data_id: None,
                    },
                ],
            }],
            version: 0,
        };

        let expected = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"params\":[{\"name\":\"port\",\"param_type\":\"string\",\"default_value\":\"80\",\"mandatory\":true}],\"args\":[{\"name\":\"method\",\"arg_type\":\"String\",\"value\":\"get\",\"data_id\":null},{\"name\":\"url\",\"arg_type\":\"String\",\"value\":\"url\",\"data_id\":null}]}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false,\"command\":\"http\"},\"credentials\":[],\"params\":[],\"data\":[],\"notifications\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"show_on_main\":false,\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"name\":\"method\",\"arg_type\":\"String\",\"value\":\"get\",\"data_id\":null},{\"name\":\"url\",\"arg_type\":\"String\",\"value\":\"url\",\"data_id\":null}]}],\"version\":0}";

        let result = serde_json::to_string(&testee).expect("should not happen");
        assert_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_deserialize_plugin() {
        let expected: Plugin = Plugin {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            server_icon: "".to_string(),
            detection: DetectionDef {
                list: vec![DetectionEntry {
                    params: vec![ParamDef {
                        name: "port".to_string(),
                        default_value: "80".to_string(),
                        param_type: "string".to_string(),
                        mandatory: true,
                    }],
                    args: vec![
                        ArgDef {
                            name: "method".to_string(),
                            arg_type: ArgType::String,
                            value: "get".to_string(),
                            data_id: None,
                        },
                        ArgDef {
                            name: "url".to_string(),
                            arg_type: ArgType::String,
                            value: "url".to_string(),
                            data_id: None,
                        },
                    ],
                }],
                script: Script {
                    script_type: "lua".to_string(),
                    script: "Dummy script".to_string(),
                },
                detection_possible: false,
                command: "http".to_string(),
            },
            credentials: vec![],
            params: vec![],
            data: vec![],
            notifications: vec![],
            actions: vec![ActionDef {
                id: "".to_string(),
                name: "".to_string(),
                show_on_main: false,
                depends: vec![],
                available_for_state: State::Any,
                needs_confirmation: false,
                description: "".to_string(),
                icon: "".to_string(),
                command: "http".to_string(),
                args: vec![
                    ArgDef {
                        name: "method".to_string(),
                        arg_type: ArgType::String,
                        value: "get".to_string(),
                        data_id: None,
                    },
                    ArgDef {
                        name: "url".to_string(),
                        arg_type: ArgType::String,
                        value: "url".to_string(),
                        data_id: None,
                    },
                ],
            }],
            version: 0,
        };

        let test_string: &str = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"params\":[{\"name\":\"port\",\"param_type\":\"string\",\"default_value\":\"80\",\"mandatory\":true}],\"args\":[{\"name\":\"method\",\"arg_type\":\"String\",\"value\":\"get\",\"data_id\":null},{\"name\":\"url\",\"arg_type\":\"String\",\"value\":\"url\",\"data_id\":null}]}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false,\"command\":\"http\"},\"credentials\":[],\"params\":[],\"data\":[],\"notifications\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"show_on_main\":false,\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"name\":\"method\",\"arg_type\":\"String\",\"value\":\"get\",\"data_id\":null},{\"name\":\"url\",\"arg_type\":\"String\",\"value\":\"url\",\"data_id\":null}]}],\"version\":0}";

        let result: Plugin = serde_json::from_str(test_string).expect("should not happen");

        assert_json_diff::assert_json_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_get_all_plugins() {
        let config = Config::builder()
            .set_default("plugin_base_path", "./shipped_plugins/plugins")
            .expect("should not happen")
            .build()
            .expect("should not happen");
        datastore::set_config(config).expect("should not happen");
        let result = init_cache();

        assert!(result.is_ok());
    }
}
