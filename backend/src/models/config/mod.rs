use serde::{Deserialize, Serialize};

use self::dns_server::DNSServer;

use super::{server::Server, users::User};

pub mod dns_server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub disabled_plugins: Vec<String>,
    pub users: Vec<User>,
    pub servers: Vec<Server>,
    pub dns_servers: Vec<DNSServer>,
}
