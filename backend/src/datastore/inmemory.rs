use crate::{
    event_handling::{self},
    models::response::{data_result::ConditionCheckResult, status::Status},
    models::{
        error::AppError,
        plugin::{monitoring::MonitioringDef, Plugin},
        server::Server,
    },
};
use config::Config;
use lazy_static::lazy_static;
use std::{collections::HashMap, net::IpAddr, sync::RwLock};

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
    static ref SERIES_TO_MONITORING: RwLock<HashMap<String, MonitioringDef>> =
        RwLock::new(HashMap::new());
}

pub fn set_config(config: Config) -> Result<(), AppError> {
    let mut holder = CONFIG
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    holder.config = Some(config);
    Ok(())
}

pub fn set_crypto_key(crypto_key: String) -> Result<(), AppError> {
    let mut holder = CONFIG
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    holder.crypto_key = Some(crypto_key);
    Ok(())
}

pub fn get_crypto_key() -> Result<String, AppError> {
    let holder = CONFIG
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;
    holder
        .crypto_key
        .as_ref()
        .map(|v| v.to_owned())
        .ok_or(AppError::Unknown(
            "Could not get crypto key from config".to_owned(),
        ))
}

pub fn get_config() -> Result<Config, AppError> {
    let holder = CONFIG
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    let res = holder
        .config
        .as_ref()
        .ok_or("could not get config".to_owned())?;

    Ok(res.clone())
}

pub fn cache_plugins(plugins: Vec<Plugin>) -> Result<usize, AppError> {
    let mut cache_rw = PLUGIN_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    let mut series_to_mon_cache_rw = SERIES_TO_MONITORING
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;

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
    Ok(plugins.len())
}

pub fn get_all_plugins() -> Result<Vec<Plugin>, AppError> {
    let cache = PLUGIN_CACHE
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    Ok(cache.values().cloned().collect())
}

pub fn get_all_plugins_map() -> Result<HashMap<String, Plugin>, AppError> {
    Ok(PLUGIN_CACHE
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?
        .clone())
}

pub fn get_plugin(id: &str) -> Result<Option<Plugin>, AppError> {
    let cache = PLUGIN_CACHE
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;
    Ok(cache.get(id).cloned())
}

pub fn clean_plugin_cache() -> Result<(), AppError> {
    PLUGIN_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?
        .clear();

    Ok(())
}

pub fn get_all_servers() -> Result<Vec<Server>, AppError> {
    let cache = SERVER_CACHE
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    Ok(cache.values().cloned().collect())
}

pub fn cache_servers(servers: Vec<Server>) -> Result<(), AppError> {
    let mut cache = SERVER_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;

    for server in servers {
        let existing = cache.insert(server.ipaddress, server.clone());

        event_handling::handle_object_change(
            Some(Box::new(server.to_owned())),
            existing.map(|old_server| Box::new(old_server) as _),
        )?;
    }
    Ok(())
}

pub fn remove_server(ipaddress: &IpAddr) -> Result<(), AppError> {
    let mut cache = SERVER_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    let mut status_cache = SERVER_STATUS_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    cache.remove(ipaddress);

    let existing = status_cache.remove(ipaddress);

    event_handling::handle_object_change(
        None,
        existing.map(|old_server| Box::new(old_server) as _),
    )?;

    Ok(())
}

pub fn add_server(server: &Server) -> Result<(), AppError> {
    let mut cache = SERVER_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;

    let existing = cache.insert(server.ipaddress, server.clone());

    event_handling::handle_object_change(
        Some(Box::new(server.to_owned())),
        existing.map(|old_server| Box::new(old_server) as _),
    )?;

    Ok(())
}

pub fn cache_status(status_list: &[Status]) -> Result<(), AppError> {
    let mut cache = SERVER_STATUS_CACHE
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    for status in status_list {
        let existing = cache.insert(status.ipaddress, status.clone());

        // Box::new(v) as _    ==>   https://stackoverflow.com/questions/69500407/trait-object-causing-type-mismatch
        event_handling::handle_object_change(
            Some(Box::new(status.to_owned())),
            existing.map(|old_status| Box::new(old_status) as _),
        )?;
    }
    Ok(())
}

pub fn get_status(ipaddress: &IpAddr) -> Result<Option<Status>, AppError> {
    let cache = SERVER_STATUS_CACHE
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    Ok(cache.get(ipaddress).cloned())
}

pub fn get_all_condition_results() -> Result<Vec<ConditionCheckResult>, AppError> {
    let cache = SERVER_ACTION_CONDITION_RESULTS
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    Ok(cache.values().cloned().collect())
}

pub fn insert_condition_result(condition_result: ConditionCheckResult) -> Result<(), AppError> {
    let mut cache = SERVER_ACTION_CONDITION_RESULTS
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;

    let existing = cache.insert(condition_result.clone().get_key(), condition_result.clone());

    event_handling::handle_object_change(
        Some(Box::new(condition_result)),
        existing.map(|old_condition_result| Box::new(old_condition_result) as _),
    )?;

    Ok(())
}

pub fn insert_token(token: &str) -> Result<(), AppError> {
    let mut store = TOKENS
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;

    store.insert(token.to_owned(), TokenInfo::new());

    Ok(())
}

pub fn delete_expired_tokens() -> Result<(), AppError> {
    let mut store = TOKENS
        .write()
        .map_err(|err| AppError::Unknown(format!("Could not get write lock. Error: {}", err)))?;
    log::debug!(
        "Number of tokens before cleanup of expired tokens {}",
        store.len()
    );
    store.retain(|_k, v| !v.is_expired());
    log::debug!(
        "Number of tokens after cleanup of expired tokens {}",
        store.len()
    );

    Ok(())
}

pub fn is_valid_token(token: &str) -> Result<bool, AppError> {
    let store = TOKENS
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    let res = match store.get(token) {
        Some(found) => !found.is_expired(),
        None => false,
    };

    Ok(res)
}

pub fn get_monitoring_config_for_series(
    series_id: &str,
) -> Result<Option<MonitioringDef>, AppError> {
    let cache = SERIES_TO_MONITORING
        .read()
        .map_err(|err| AppError::Unknown(format!("Could not get read lock. Error: {}", err)))?;

    Ok(cache.get(series_id).cloned())
}
