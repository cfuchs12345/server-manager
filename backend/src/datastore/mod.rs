mod config;
mod inmemory;
mod model;
mod notifications;
mod persistence;
mod plugins;
mod servers;
mod timeseries;
mod users;

use lazy_static::lazy_static;
pub use model::migration::Migration;
pub use model::Entry;
pub use timeseries::TimeSeriesPersistence;

pub use std::sync::atomic::{AtomicBool, Ordering};

pub use self::config::delete_dnsserver;
pub use self::config::get_all_dnsservers;
pub use self::config::get_encryption_key;
pub use self::config::insert_dnsserver;
pub use self::config::insert_new_encryption_key;
pub use self::config::upate_dnsserver;

pub use self::config::export_config;
pub use self::config::import_config;

pub use self::plugins::disable_plugins;
pub use self::plugins::get_disabled_plugins;
pub use self::plugins::is_plugin_disabled;
pub use self::plugins::load_plugin;

pub use self::inmemory::get_all_servers as get_all_servers_from_cache;
pub use self::servers::delete_server;
pub use self::servers::get_all_servers;
pub use self::servers::get_server;
pub use self::servers::insert_server;
pub use self::servers::re_encrypt_server;
pub use self::servers::re_encrypt_servers;
pub use self::servers::simplify_server_for_client;
pub use self::servers::simplify_servers_for_client;
pub use self::servers::update_server;

pub use self::users::decrypt_users;
pub use self::users::delete_user;
pub use self::users::encrypt_users;
pub use self::users::get_all_users;
pub use self::users::get_user;
pub use self::users::insert_user;
pub use self::users::update_user;

pub use self::notifications::get_all_notifications;
pub use self::notifications::insert_notifications;
pub use self::notifications::insert_or_update_notifications;

pub use self::inmemory::delete_expired_tokens;
pub use self::inmemory::insert_token;
pub use self::inmemory::is_valid_token;

pub use self::inmemory::cache_plugins;
pub use self::inmemory::cache_servers;
pub use self::inmemory::cache_status;
pub use self::inmemory::clean_plugin_cache;
pub use self::inmemory::get_all_condition_results;
pub use self::inmemory::get_all_plugins;
pub use self::inmemory::get_all_plugins_map;
pub use self::inmemory::get_config;
pub use self::inmemory::get_crypto_key;
pub use self::inmemory::get_monitoring_config_for_series;
pub use self::inmemory::get_plugin;
pub use self::inmemory::get_status;
pub use self::inmemory::insert_condition_result;
pub use self::inmemory::set_config;
pub use self::inmemory::set_crypto_key;

pub use self::persistence::init_db;
pub use self::persistence::save_migrations;

pub use self::timeseries::get_timeseriesdb_config;
pub use self::timeseries::save_timeseries_data;

pub use crate::models::timeseries::TimeSeriesData;

pub fn init_cache() {
    if let Ok(number) = plugins::init_cache() {
        log::debug!("Loaded {} plugins into cache", number);
    };
}

pub fn update_cache() {
    plugins::init_cache_silent();
}
