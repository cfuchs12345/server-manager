mod init;

mod upnp;
mod systeminfo;

mod webserver;
mod commands;
mod plugin_execution;
mod datastore;
mod migrations;
mod common;
mod models;

#[actix_web::main]
pub async fn main() {
    let result = init::start().await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}