use config::Config;
use lazy_static::lazy_static;
use std::{collections::HashMap, net::IpAddr, sync::RwLock};

use crate::{
    models::response::{data_result::ConditionCheckResult, status::Status},
    models::{
        plugin::{data::Monitioring, Plugin},
        server::Server,
    },
};

use crate::models::token::TokenInfo;

struct ConfigHolder {
    config: Option<Config>,
    crypto_key: Option<String>,
}

impl ConfigHolder {
    pub fn new() -> ConfigHolder {
        ConfigHolder {
            config: None,
            crypto_key: None,
        }
    }
}

lazy_static! {
    static ref TOKENS: RwLock<HashMap<String, TokenInfo>> = RwLock::new(HashMap::new());
    static ref CONFIG: RwLock<ConfigHolder> = RwLock::new(ConfigHolder::new());
    static ref PLUGIN_CACHE: RwLock<HashMap<String, Plugin>> = RwLock::new(HashMap::new());
    static ref SERVER_CACHE: RwLock<HashMap<IpAddr, Server>> = RwLock::new(HashMap::new());
    static ref SERVER_STATUS_CACHE: RwLock<HashMap<IpAddr, Status>> = RwLock::new(HashMap::new());
    static ref SERVER_ACTION_CONDITION_RESULTS: RwLock<HashMap<String, ConditionCheckResult>> =
        RwLock::new(HashMap::new());
    static ref SERIES_TO_MONITORING: RwLock<HashMap<String, Monitioring>> =
        RwLock::new(HashMap::new());
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
    let mut series_to_mon_cache_rw = SERIES_TO_MONITORING.try_write().unwrap();

    let mut to_remove: Vec<String> = Vec::new();

    for plugin_in_cache in cache_rw.values() {
        if !plugins.iter().any(|p| p.id == plugin_in_cache.id) {
            to_remove.push(plugin_in_cache.id.clone());
        }
    }

    for plugin in &plugins {
        cache_rw.insert(plugin.id.clone(), plugin.clone());

        plugin.data.iter().for_each(|d| {
            d.monitoring.iter().for_each(|m| {
                series_to_mon_cache_rw.insert(m.id.to_owned(), m.clone());
            })
        })
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
        cache.insert(server.ipaddress, server);
    }
}

pub fn remove_server(ipaddress: &IpAddr) {
    let mut cache = SERVER_CACHE.try_write().unwrap();
    let mut status_cache = SERVER_STATUS_CACHE.try_write().unwrap();
    cache.remove(ipaddress);
    status_cache.remove(ipaddress);
}

pub fn add_server(server: &Server) {
    let mut cache = SERVER_CACHE.try_write().unwrap();

    cache.insert(server.ipaddress, server.clone());
}

pub fn cache_status(status: &[Status]) {
    let mut cache = SERVER_STATUS_CACHE.try_write().unwrap();
    for s in status {
        cache.insert(s.ipaddress, s.to_owned());
    }
}

pub fn get_status(ipaddress: &IpAddr) -> Option<Status> {
    let cache = SERVER_STATUS_CACHE.try_read().unwrap();

    cache.get(ipaddress).cloned()
}

pub fn get_all_condition_results() -> Vec<ConditionCheckResult> {
    let cache = SERVER_ACTION_CONDITION_RESULTS.try_read().unwrap();

    cache.values().cloned().collect()
}

pub fn insert_condition_result(to_add: ConditionCheckResult) {
    let mut cache = SERVER_ACTION_CONDITION_RESULTS.try_write().unwrap();

    cache.insert(to_add.clone().get_key(), to_add);
}

pub fn insert_token(token: &str) {
    let mut store = TOKENS.try_write().unwrap();

    store.insert(token.to_owned(), TokenInfo::new());
}

pub fn delete_expired_tokens() {
    let mut store = TOKENS.try_write().unwrap();
    log::debug!(
        "Number of tokens before cleanup of expired tokens {}",
        store.len()
    );
    store.retain(|_k, v| !v.is_expired());
    log::debug!(
        "Number of tokens after cleanup of expired tokens {}",
        store.len()
    );
}

pub fn is_valid_token(token: &str) -> bool {
    let store = TOKENS.try_read().unwrap();

    match store.get(token) {
        Some(found) => !found.is_expired(),
        None => false,
    }
}

pub fn get_monitoring_config_for_series(series_id: &str) -> Option<Monitioring> {
    let cache = SERIES_TO_MONITORING.try_read().unwrap();

    cache.get(series_id).cloned()
}
