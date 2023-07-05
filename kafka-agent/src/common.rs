use std::time::Duration;

use lazy_static::lazy_static;

use crate::systeminfo;

pub const TEN_SECS: std::time::Duration = std::time::Duration::from_secs(10);
pub const FIVE_SECS: std::time::Duration = std::time::Duration::from_secs(5);

pub const BROKER_ENV_VAR: &str = "BROKER";
pub const CONSUME_TOPIC_ENV_VAR: &str = "CONSUME_TOPIC";
pub const PUBLISH_TOPIC_ENV_VAR: &str = "PUBLISH_TOPIC";
pub const ERROR_TOPIC: &str = "ERROR_TOPIC";
pub const REGISTRATION_TOPIC: &str = "REGISTRATION_TOPIC";
pub const HEARTBEAT_TOPIC: &str = "HEARTBEAT_TOPIC";
pub const GROUP_VAR: &str = "GROUP";

lazy_static! {
    pub static ref IS_WINDOWS: bool = cfg!(target_os = "windows");
    pub static ref CLIENT_ID: String = systeminfo::get_system_info().get_host_name();
}

pub fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}

pub fn get_env_var(name: &str) -> String {
    match std::env::var(name.to_uppercase()) {
        Ok(value) => value,
        Err(_) => {
            let message = format!("env var {} not set. Please check that the .env file is present and contains the setting {}", name, name);
            println!("{}", message);
            panic!("{}", message);
        }
    }
}
