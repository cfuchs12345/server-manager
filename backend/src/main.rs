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
mod handlebars_helpers;


pub fn main() {
    dotenvy::dotenv().ok(); 

    env_logger::init();

    let result = init::start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}