mod commands;
mod common;
mod datastore;
mod init;
mod migrations;
mod models;
mod other_functions;
mod plugin_execution;
mod webserver;

#[actix_web::main]
pub async fn main() {
    let result = init::start().await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
