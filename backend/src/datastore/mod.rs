mod config;
mod inmemory;
mod model;
mod persistence;
mod plugins;
mod servers;
mod timeseries_persistence;
mod users;

pub use model::migration::Migration;
pub use model::Entry;
pub use persistence::Persistence;
pub use timeseries_persistence::TimeSeriesPersistence;

pub use self::config::delete_dnsserver;
pub use self::config::get_all_dnsservers;
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

use crate::models::error::AppError;

pub use self::timeseries_persistence::QuestDBConfig;
pub use self::timeseries_persistence::TimeSeriesData;
pub use self::timeseries_persistence::TimeSeriesValue;
pub use self::timeseries_persistence::Timestamp;

pub fn init_cache() {
    if let Ok(number) = plugins::init_cache() {
        log::debug!("Loaded {} plugins into cache", number);
    };
}

pub fn update_cache() {
    plugins::init_cache_silent();
}

pub async fn save_timeseries_data(
    timeseries_persistence: &mut TimeSeriesPersistence,
    series_id: &str,
    data_vec: Vec<TimeSeriesData>,
) -> Result<(), AppError> {
    timeseries_persistence.save(series_id, data_vec).await
}
