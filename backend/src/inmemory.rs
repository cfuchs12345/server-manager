use std::{collections::HashMap, sync::Mutex};
use config::Config;
use lazy_static::lazy_static;

use crate::{plugin_types::Plugin, types::{Status}};

struct ConfigHolder {
    config: Option<Config>
}


impl ConfigHolder {
    pub fn new() -> ConfigHolder {
        ConfigHolder {
            config: None
        }
    }
}

lazy_static!(
    static ref CONFIG:Mutex<ConfigHolder> = Mutex::new(ConfigHolder::new());
    static ref PLUGIN_CACHE: Mutex<HashMap<String, Plugin>> = Mutex::new(HashMap::new());
    static ref STATUS_CACHE: Mutex<HashMap<String, Status>> = Mutex::new(HashMap::new());
);


pub fn set_config(config: Config) {
    let mut holder = CONFIG.lock().unwrap();
    holder.config=Some(config);
}

pub fn get_config() -> Config {
    let holder = CONFIG.lock().unwrap();
    
    let res = holder.config.as_ref().unwrap();

    res.clone()
}

pub fn insert_plugins(plugins : Vec<Plugin>) {
    let mut cache = PLUGIN_CACHE.lock().unwrap();

    for plugin in plugins {
        cache.insert(plugin.id.clone(), plugin);
    }
}

pub fn get_all_plugins() -> Vec<Plugin> {
    let cache = PLUGIN_CACHE.lock().unwrap();
    
    
    cache.values().map( |v| v.clone()).collect()
}

pub fn get_all_plugins_map() -> HashMap<String, Plugin>{
    PLUGIN_CACHE.lock().unwrap().clone()
}

pub fn get_plugin( id: &str) -> Option<Plugin> {
    let cache = PLUGIN_CACHE.lock().unwrap();
    let res = cache.get(id);
    match res {
        Some(val) => Some(val.clone()),
        None => None
    }
}

pub fn clean_plugin_cache() {
    PLUGIN_CACHE.lock().unwrap().clear();
}


