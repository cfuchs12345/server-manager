use std::time::Duration;

use lazy_static::lazy_static;

use crate::systeminfo;

pub const TEN_SECS: std::time::Duration = std::time::Duration::from_secs(10);
pub const FIVE_SECS: std::time::Duration = std::time::Duration::from_secs(5);

pub const BROKER_ENV_VAR: &str = "broker";
pub const CONSUME_TOPIC_ENV_VAR: &str = "consume_topic";
pub const PUBLISH_TOPIC_ENV_VAR: &str = "publish_topic";
pub const ERROR_TOPIC: &str = "error_topic";
pub const REGISTRATION_TOPIC: &str = "registration_topic";
pub const HEARTBEAT_TOPIC: &str = "heartbeat_topic";
pub const GROUP_VAR: &str = "group";

lazy_static! {
    pub static ref IS_WINDOWS: bool = cfg!(target_os = "windows");
    pub static ref CLIENT_ID: String = systeminfo::get_system_info().get_host_name();
}

pub fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}

pub fn get_env_var(name: &str) -> String {
    std::env::var(name).unwrap()
}
