mod appdata;
mod config;
mod discover;
mod errors;
mod init;
mod persistence;
mod plugins;
mod routes;
mod servers;
mod types;
mod features;
mod plugin_types;
mod server_types;
mod config_types;
mod conversion;
mod handlebars_helper_functions;
mod status;
mod crypt;
mod migrations;
mod upnp;
mod http_functions;
mod inmemory;
mod scheduler;

pub fn main() {
    let result = init::start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}