use config::Config;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

use crate::{plugin_types::Plugin, server_types::Server, types::Status};

struct ConfigHolder {
    config: Option<Config>,
}

impl ConfigHolder {
    pub fn new() -> ConfigHolder {
        ConfigHolder { config: None }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<ConfigHolder> = Mutex::new(ConfigHolder::new());
    static ref PLUGIN_CACHE: Mutex<HashMap<String, Plugin>> = Mutex::new(HashMap::new());
    static ref SERVER_CACHE: Mutex<HashMap<String, Server>> = Mutex::new(HashMap::new());
    static ref SERVER_STATUS_CACHE: Mutex<HashMap<String, Status>> = Mutex::new(HashMap::new());
}

pub fn set_config(config: Config) {
    let mut holder = CONFIG.lock().unwrap();
    holder.config = Some(config);
}

pub fn get_config() -> Config {
    let holder = CONFIG.lock().unwrap();

    let res = holder.config.as_ref().unwrap();

    res.clone()
}

pub fn insert_plugins(plugins: Vec<Plugin>) {
    let mut cache = PLUGIN_CACHE.lock().unwrap();

    for plugin in plugins {
        cache.insert(plugin.id.clone(), plugin);
    }
}

pub fn get_all_plugins() -> Vec<Plugin> {
    let cache = PLUGIN_CACHE.lock().unwrap();

    cache.values().cloned().collect()
}

pub fn get_all_plugins_map() -> HashMap<String, Plugin> {
    PLUGIN_CACHE.lock().unwrap().clone()
}

pub fn get_plugin(id: &str) -> Option<Plugin> {
    let cache = PLUGIN_CACHE.lock().unwrap();
    cache.get(id).cloned()
}

pub fn clean_plugin_cache() {
    PLUGIN_CACHE.lock().unwrap().clear();
}

pub fn get_all_servers() -> Vec<Server> {
    let cache = SERVER_CACHE.lock().unwrap().clone();

    cache.values().cloned().collect()
}

pub fn insert_servers(servers: Vec<Server>) {
    let mut cache = SERVER_CACHE.lock().unwrap();

    for server in servers {
        cache.insert(server.ipaddress.clone(), server);
    }
}

pub fn remove_server(ipaddress:  &str) {
    let mut cache = SERVER_CACHE.lock().unwrap();
    let mut status_cache = SERVER_STATUS_CACHE.lock().unwrap();
    cache.remove(ipaddress);
    status_cache.remove(ipaddress);
}

pub fn add_server(server: &Server) {
    let mut cache = SERVER_CACHE.lock().unwrap();

    cache.insert(server.ipaddress.clone(), server.clone());
}

pub fn insert_status(status: Vec<Status>) {
    let mut cache = SERVER_STATUS_CACHE.lock().unwrap();
    for s in status {
        cache.insert(s.ipaddress.clone(), s);
    }
}

pub fn get_status(ipaddress: String) -> Option<Status> {
    let cache = SERVER_STATUS_CACHE.lock().unwrap();

    cache.get(ipaddress.as_str()).cloned()
}
