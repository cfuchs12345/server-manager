use futures::future::join_all;
use futures::lock::Mutex;
use lazy_static::lazy_static;
use log::debug;

use std::collections::HashMap;
use std::{fs::File, io::BufReader};

use crate::datastore;
use crate::models::error::AppError;
use crate::models::plugin::Plugin;

use super::persistence::Persistence;
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
        let file_name = os_string.to_str().unwrap();
        plugin_file_names.push(file_name.to_string());
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
    let len = plugins.len();

    datastore::cache_plugins(plugins);
    Ok(len)
}

async fn load_all() -> Result<Vec<Plugin>, AppError> {
    datastore::clean_plugin_cache();
    let plugin_base_path = datastore::get_config()
        .get_string("plugin_base_path")
        .unwrap();

    let plugin_file_names = get_all_plugin_filenames(plugin_base_path.as_str())?;

    let plugins: Vec<Plugin> = join_all(plugin_file_names.iter().map(|plugin_file_name| async {
        let plugin = load_plugin(plugin_base_path.as_str(), plugin_file_name)
            .await
            .unwrap();

        PLUGIN_NAME_TO_FILENAME
            .lock()
            .await
            .insert(plugin.id.clone(), plugin_file_name.to_owned());
        plugin
    }))
    .await;

    Ok(plugins)
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

pub async fn get_disabled_plugins(persistence: &Persistence) -> Result<Vec<String>, AppError> {
    match persistence.get(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(res) => match res {
            Some(entry) => Ok(entry.value.split(',').map(|e| e.to_string()).collect()),
            None => Ok(Vec::new()),
        },
        Err(err) => Err(AppError::from(err)),
    }
}

pub async fn disable_plugins(
    persistence: &Persistence,
    plugin_ids: Vec<String>,
) -> Result<bool, AppError> {
    match persistence
        .delete(TABLE_PLUGIN_CONFIG, "disabled_ids")
        .await
    {
        Ok(_res) => {
            match persistence
                .insert(
                    TABLE_PLUGIN_CONFIG,
                    Entry {
                        key: "disabled_ids".to_string(),
                        value: plugin_ids.join(",").to_string(),
                    },
                )
                .await
            {
                Ok(_res) => Ok(true),
                Err(err) => Err(AppError::from(err)),
            }
        }
        Err(err) => Err(AppError::from(err)),
    }
}

pub async fn is_plugin_disabled(
    plugin_id: &str,
    persistence: &Persistence,
) -> Result<bool, AppError> {
    match persistence.get(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(res) => {
            match res {
                Some(entry) => {
                    let mut ids = entry.value.split(',');
                    Ok(ids.any(|id| *id == *plugin_id))
                }
                None => Ok(false), // default is that it is activated
            }
        }
        Err(err) => Err(AppError::from(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plugin::{
        action::{Action, State},
        common::{ArgDef, Script},
        detection::{Detection, DetectionEntry},
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
            detection: Detection {
                list: vec![DetectionEntry {
                    params: vec![ParamDef {
                        name: "port".to_string(),
                        default_value: "80".to_string(),
                        param_type: "string".to_string(),
                        mandatory: true,
                    }],
                    args: vec![
                        ArgDef {
                            arg_type: "method".to_string(),
                            value: "get".to_string(),
                        },
                        ArgDef {
                            arg_type: "url".to_string(),
                            value: "url".to_string(),
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
            actions: vec![Action {
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
                        arg_type: "method".to_string(),
                        value: "get".to_string(),
                    },
                    ArgDef {
                        arg_type: "url".to_string(),
                        value: "url".to_string(),
                    },
                ],
            }],
        };

        let expected = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"params\":[{\"name\":\"port\",\"param_type\":\"string\",\"default_value\":\"80\",\"mandatory\":true}],\"args\":[{\"arg_type\":\"method\",\"value\":\"get\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false,\"command\":\"http\"},\"credentials\":[],\"params\":[],\"data\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"show_on_main\":false,\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"arg_type\":\"method\",\"value\":\"get\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}]}";

        let result = serde_json::to_string(&testee).unwrap();
        assert_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_deserialize_plugin() {
        let expected: Plugin = Plugin {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            server_icon: "".to_string(),
            detection: Detection {
                list: vec![DetectionEntry {
                    params: vec![ParamDef {
                        name: "port".to_string(),
                        default_value: "80".to_string(),
                        param_type: "string".to_string(),
                        mandatory: true,
                    }],
                    args: vec![
                        ArgDef {
                            arg_type: "method".to_string(),
                            value: "get".to_string(),
                        },
                        ArgDef {
                            arg_type: "url".to_string(),
                            value: "url".to_string(),
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
            actions: vec![Action {
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
                        arg_type: "method".to_string(),
                        value: "get".to_string(),
                    },
                    ArgDef {
                        arg_type: "url".to_string(),
                        value: "url".to_string(),
                    },
                ],
            }],
        };

        let test_string: &str = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"params\":[{\"name\":\"port\",\"param_type\":\"string\",\"default_value\":\"80\",\"mandatory\":true}],\"args\":[{\"arg_type\":\"method\",\"value\":\"get\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false,\"command\":\"http\"},\"credentials\":[],\"params\":[],\"data\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"show_on_main\":false,\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"arg_type\":\"method\",\"value\":\"get\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}]}";

        let result: Plugin = serde_json::from_str(test_string).unwrap();

        assert_json_diff::assert_json_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_get_all_plugins() {
        let config = Config::builder()
            .set_default("plugin_base_path", "./shipped_plugins/plugins")
            .unwrap()
            .build()
            .unwrap();
        datastore::set_config(config);
        let result = init_cache();

        assert!(result.is_ok());
    }
}
