use futures::future::join_all;
use futures::lock::Mutex;
use lazy_static::lazy_static;
use log::debug;
use rlua::Lua;
use std::collections::HashMap;
use std::io::Error;
use std::{fs::File, io::BufReader};

use crate::persistence::{Persistence, Entry};
use crate::plugin_types::Plugin;

const TABLE_PLUGIN_CONFIG: &'static str = "plugin_config";

lazy_static! {
    static ref  PLUGIN_NAME_TO_FILENAME: Mutex<HashMap<String,String>> =  Mutex::new(HashMap::new());
}

pub fn get_all_plugin_filenames(plugin_base_path: &str) -> Result<Vec<String>, Error> {
    let mut plugin_file_names: Vec<String> = vec![];

    let paths = std::fs::read_dir(plugin_base_path)?;

    for path in paths {
        let os_string = path?.file_name();
        let file_name = os_string.to_str().unwrap();
        plugin_file_names.push(file_name.to_string());
    }
    Ok(plugin_file_names)
}


pub async fn get_filename_for_plugin(feature_id: String, plugin_base_path: &str) -> Option<String> {
    if PLUGIN_NAME_TO_FILENAME.lock().await.len() == 0 {
        match get_all_plugins(plugin_base_path).await {
            Ok(_res) => {},
            Err(err) => {
                log::error!("Could not load plugins. Error was {}", err);
            }
        }
    }

    PLUGIN_NAME_TO_FILENAME.lock().await.get(&feature_id).map( |val| val.clone())
 }




pub async fn get_all_plugins(plugin_base_path: &str) -> Result<Vec<Plugin>, Error> {
    let plugin_file_names = get_all_plugin_filenames(plugin_base_path)?;

    let plugins: Vec<Plugin> = join_all(plugin_file_names.iter().map(|plugin_file_name| async {
        let plugin = load_plugin(&plugin_base_path, plugin_file_name)
            .await
            .unwrap();

            PLUGIN_NAME_TO_FILENAME.lock().await.insert(plugin.id.clone(), plugin_file_name.to_owned());
        return plugin;
    }))
    .await;

    debug!("Plugins loaded: {:?}", plugins);
    Ok(plugins)
}

pub fn plugin_detect_match(plugin: &Plugin, input: &str) -> Result<bool, Error> {
    let script = plugin.detection.script.script.clone();
    let script_type = plugin.detection.script.script_type.clone();

    let is_lua = match script_type.as_str() {
        "lua" => true,
        _ => false,
    };

    if !is_lua {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Only LUA scripts are currently supported",
        ));
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

pub async fn load_plugin(plugin_base_path: &str, plugin_file_name: &str) -> Result<Plugin, Error> {
    match File::open(plugin_base_path.to_owned() + "/" + plugin_file_name) {
        Ok(file) => {
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            match serde_json::from_reader(reader) {
                Ok::<Plugin, _>(plugin) => {
                    log::debug!("plugin loaded: {:?}", plugin);
                    return Ok(plugin);
                }
                Err(err) => {
                    log::error!("Error while parsing plugin file {} was: {}", plugin_file_name,  err);
                    Err(Error::from(err))
                }
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn get_disabled_plugins(persistence: &Persistence) -> Result<Vec<String>, Error>{
    match persistence.get(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(res) => Ok(res.value.split(",").into_iter().map( |e| e.to_string()).collect()),
        Err(_err) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Could not load disabled plugins from database",
            ));
        }
    }
}

pub async fn disable_plugins(persistence: &Persistence, plugin_ids: Vec<String>) -> Result<bool, Error> {
    match persistence.delete(TABLE_PLUGIN_CONFIG, "disabled_ids").await {
        Ok(_res) => {
            match persistence.insert(TABLE_PLUGIN_CONFIG, Entry {
                key: "disabled_ids".to_string(),
                value: plugin_ids.join(",").to_string()
            }).await {
                Ok(_res) => {
                    return Ok(true);
                },
                Err(_err) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Could not insert entry for disabled plugins",
                    ));
                }
            }                                
        },
        Err(_err) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Could not delete old entry for disabled plugins",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin_types::{ArgDef, Detection, Script, DetectionEntry, Action, State};

    use super::*;

    #[test]
    fn test_serialize_plugin() {
        let testee: Plugin = Plugin {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            server_icon: "".to_string(),
            detection: Detection {
                list: vec![DetectionEntry {
                    defaultports: vec![80, 81],
                    url: "http://${IP}:${PORT}".to_string(),
                }],
                script: Script {
                    script_type: "lua".to_string(),
                    script: "Dummy script".to_string(),
                },
                detection_possible: false,
            },
            credentials: vec![],
            params: vec![],
            data: vec![],
            actions: vec![Action {
                id: "".to_string(),
                name: "".to_string(),
                depends: vec![],
                available_for_state: State::Any,
                needs_confirmation: false,
                description: "".to_string(),
                icon: "".to_string(),
                command: "http".to_string(),
                args: vec![
                    ArgDef {
                        arg_type: "method".to_string(),
                        value: "GET".to_string(),
                    },
                    ArgDef {
                        arg_type: "url".to_string(),
                        value: "url".to_string(),
                    },
                ],
            }],
        };

        let expected = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"defaultports\":[80,81],\"url\":\"http://${IP}:${PORT}\"}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false},\"credentials\":[],\"params\":[],\"data\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"arg_type\":\"method\",\"value\":\"GET\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}]}";

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
                    defaultports: vec![80, 81],
                    url: "http://${IP}:${PORT}".to_string(),
                }],
                script: Script {
                    script_type: "lua".to_string(),
                    script: "Dummy script".to_string(),
                },
                detection_possible: false,
            },
            credentials: vec![],
            params: vec![],
            data: vec![],
            actions: vec![Action {
                id: "".to_string(),
                name: "".to_string(),
                depends: vec![],
                available_for_state: State::Any,
                needs_confirmation: false,
                description: "".to_string(),
                icon: "".to_string(),
                command: "http".to_string(),
                args: vec![
                    ArgDef {
                        arg_type: "method".to_string(),
                        value: "GET".to_string(),
                    },
                    ArgDef {
                        arg_type: "url".to_string(),
                        value: "url".to_string(),
                    },
                ],
            }],
        };

        let test_string: &str = "{\"id\":\"test\",\"name\":\"Test\",\"description\":\"\",\"server_icon\":\"\",\"detection\":{\"list\":[{\"defaultports\":[80,81],\"url\":\"http://${IP}:${PORT}\"}],\"script\":{\"script_type\":\"lua\",\"script\":\"Dummy script\"},\"detection_possible\":false},\"credentials\":[],\"params\":[],\"data\":[],\"actions\":[{\"id\":\"\",\"name\":\"\",\"depends\":[],\"available_for_state\":\"Any\",\"needs_confirmation\":false,\"description\":\"\",\"icon\":\"\",\"command\":\"http\",\"args\":[{\"arg_type\":\"method\",\"value\":\"GET\"},{\"arg_type\":\"url\",\"value\":\"url\"}]}]}";

        let result: Plugin = serde_json::from_str(&test_string).unwrap();

        assert_json_diff::assert_json_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_get_all_plugins() {
        let result = get_all_plugins("shipped_plugins/plugins").await;

        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_match() {
        let input = "<result>\
                <application>sleep-on-lan</application>\
                <version>1.1.1-RELEASE</version>\
                <compilation-timestamp>2022-08-13T22:25:28+0200</compilation-timestamp>\
                <commit>35982e56d2bf98f27afb01a2cfc793754af8d3da</commit>\
                <hosts>\
                <host ip=\"127.0.0.1/8\" mac=\"\" reversed-mac=\"\"/>\
                <host ip=\"192.168.178.20/24\" mac=\"6c:4b:90:66:3b:91\" reversed-mac=\"91:3b:66:90:4b:6c\"/>\
                <host ip=\"192.168.179.2/24\" mac=\"00:00:00:00:1a:54\" reversed-mac=\"54:1a:00:00:00:00\"/>\
                <host ip=\"192.168.222.1/24\" mac=\"12:af:1a:8a:dc:96\" reversed-mac=\"96:dc:8a:1a:af:12\"/>\
                </hosts>\
                <listeners>\
                <listener type=\"UDP\" port=\"9\" active=\"true\"/>\
                <listener type=\"HTTP\" port=\"8009\" active=\"true\"/>\
                </listeners>\
                <commands>\
                <command operation=\"sleep\" command=\"systemctl suspend\" default=\"true\" type=\"external\"/>\
                <command operation=\"shutdown\" command=\"shutdown -h\" default=\"false\" type=\"external\"/>\
                </commands>\
                </result>";

        let plugin = load_plugin("shipped_plugins/plugins", "sleep.json").await;

        let result = plugin_detect_match(&plugin.unwrap(), input);

        assert_eq!(true, result.unwrap());
    }
}
