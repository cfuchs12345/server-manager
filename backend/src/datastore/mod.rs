mod persistence;
mod config;
mod inmemory;
mod servers;
mod plugins;
mod users;
mod model;

pub use persistence::Persistence;
pub use model::Entry;
pub use model::migration::Migration;

pub use crate::datastore::config::load_all_dnsservers;
pub use crate::datastore::config::insert_dnsserver;
pub use crate::datastore::config::delete_dnsserver;
pub use crate::datastore::config::insert_new_encryption_key;

pub use crate::datastore::plugins::is_plugin_disabled;
pub use crate::datastore::plugins::get_disabled_plugins;
pub use crate::datastore::plugins::disable_plugins;
pub use crate::datastore::plugins::load_plugin;

pub use crate::datastore::servers::load_all_servers;
pub use crate::datastore::servers::get_server;
pub use crate::datastore::servers::insert_server;
pub use crate::datastore::servers::delete_server;
pub use crate::datastore::servers::update_server;

pub use crate::datastore::users::insert_user;
pub use crate::datastore::users::update_user;
pub use crate::datastore::users::delete_user;
pub use crate::datastore::users::load_all_users;
pub use crate::datastore::users::get_user;

pub use crate::datastore::inmemory::is_valid_token;
pub use crate::datastore::inmemory::insert_token;
pub use crate::datastore::inmemory::delete_expired_tokens;

pub use crate::datastore::inmemory::get_all_plugins;
pub use crate::datastore::inmemory::get_all_servers;
pub use crate::datastore::inmemory::get_all_condition_results;
pub use crate::datastore::inmemory::insert_condition_result;
pub use crate::datastore::inmemory::cache_status;
pub use crate::datastore::inmemory::cache_plugins;
pub use crate::datastore::inmemory::cache_servers;
pub use crate::datastore::inmemory::get_config;
pub use crate::datastore::inmemory::get_plugin;
pub use crate::datastore::inmemory::set_config;
pub use crate::datastore::inmemory::get_crypto_key;
pub use crate::datastore::inmemory::set_crypto_key;
pub use crate::datastore::inmemory::get_all_plugins_map;
pub use crate::datastore::inmemory::get_status;
pub use crate::datastore::inmemory::clean_plugin_cache;

pub fn init_cache() {
    if let Ok(number) = plugins::init_cache() {
        log::debug!("Loaded {} plugins into cache", number);
    };
}

pub fn update_cache() {
    plugins::init_cache_silent();
}


