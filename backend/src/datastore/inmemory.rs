use config::Config;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};

use crate::{models::{plugin::Plugin, server::Server}, models::response::{status::Status, data_result::ConditionCheckResult}};


struct ConfigHolder {
    config: Option<Config>,
    crypto_key: Option<String>
}

impl ConfigHolder {
    pub fn new() -> ConfigHolder {
        ConfigHolder { config: None, crypto_key: None }
    }
}

lazy_static! {
    static ref CONFIG: RwLock<ConfigHolder> = RwLock::new(ConfigHolder::new());
    static ref PLUGIN_CACHE: RwLock<HashMap<String, Plugin>> = RwLock::new(HashMap::new());
    static ref SERVER_CACHE: RwLock<HashMap<String, Server>> = RwLock::new(HashMap::new());
    static ref SERVER_STATUS_CACHE: RwLock<HashMap<String, Status>> = RwLock::new(HashMap::new());
    static ref SERVER_ACTION_CONDITION_RESULTS: RwLock<HashMap<String, ConditionCheckResult>> = RwLock::new(HashMap::new());
}

pub fn set_config(config: Config) {
    let mut holder = CONFIG.try_write().unwrap();
    holder.config = Some(config);
}

pub fn set_crypto_key(crypto_key: String) {
    let mut holder = CONFIG.try_write().unwrap();
    holder.crypto_key = Some(crypto_key);
}

pub fn get_crypto_key() -> String {
    let holder = CONFIG.try_read().unwrap();
    holder.crypto_key.as_ref().unwrap().clone()
}

pub fn get_config() -> Config {
    let holder = CONFIG.try_read().unwrap();

    let res = holder.config.as_ref().unwrap();

    res.clone()
}

pub fn cache_plugins(plugins: Vec<Plugin>) {
    let mut cache_rw = PLUGIN_CACHE.try_write().unwrap();
    let mut to_remove: Vec<String> = Vec::new();
    
    for plugin_in_cache in cache_rw.values() {
        if !plugins.iter().any(|p| p.id == plugin_in_cache.id) {
        to_remove.push(plugin_in_cache.id.clone());
        }
    }
        
    for plugin in &plugins {
        cache_rw.insert(plugin.id.clone(), plugin.clone());
    }
    for id_to_remove in to_remove {
        cache_rw.remove(id_to_remove.as_str());
    }
}



pub fn get_all_plugins() -> Vec<Plugin> {
    let cache = PLUGIN_CACHE.try_read().unwrap();

    cache.values().cloned().collect()
}

pub fn get_all_plugins_map() -> HashMap<String, Plugin> {
    PLUGIN_CACHE.try_read().unwrap().clone()
}

pub fn get_plugin(id: &str) -> Option<Plugin> {
    let cache = PLUGIN_CACHE.try_read().unwrap();
    cache.get(id).cloned()
}

pub fn clean_plugin_cache() {
    PLUGIN_CACHE.try_write().unwrap().clear();
}

pub fn get_all_servers() -> Vec<Server> {
    let cache = SERVER_CACHE.try_read().unwrap().clone();

    cache.values().cloned().collect()
}

pub fn cache_servers(servers: Vec<Server>) {
    let mut cache = SERVER_CACHE.try_write().unwrap();

    for server in servers {
        cache.insert(server.ipaddress.clone(), server);
    }
}

pub fn remove_server(ipaddress:  &str) {
    let mut cache = SERVER_CACHE.try_write().unwrap();
    let mut status_cache = SERVER_STATUS_CACHE.try_write().unwrap();
    cache.remove(ipaddress);
    status_cache.remove(ipaddress);
}

pub fn add_server(server: &Server) {
    let mut cache = SERVER_CACHE.try_write().unwrap();

    cache.insert(server.ipaddress.clone(), server.clone());
}

pub fn cache_status(status: Vec<Status>) {
    let mut cache = SERVER_STATUS_CACHE.try_write().unwrap();
    for s in status {
        cache.insert(s.ipaddress.clone(), s);
    }
}

pub fn get_status(ipaddress: String) -> Option<Status> {
    let cache = SERVER_STATUS_CACHE.try_read().unwrap();

    cache.get(ipaddress.as_str()).cloned()
}

pub fn get_all_condition_results() -> Vec<ConditionCheckResult> {
    let cache = SERVER_ACTION_CONDITION_RESULTS.try_read().unwrap();

    cache.values().cloned().collect()
}

pub fn insert_condition_result(to_add: ConditionCheckResult) {
    let mut cache = SERVER_ACTION_CONDITION_RESULTS.try_write().unwrap();
    
    cache.insert(to_add.clone().get_key(), to_add);
}
