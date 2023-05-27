
use std::net::IpAddr;

use crate::models::{server::Server, error::AppError};

use super::{persistence::{Persistence}, inmemory, Entry};


const TABLE: &str = "servers";

fn json_to_server(json: &str) -> Server {
    serde_json::from_str(json).unwrap()
}

fn entries_to_servers(jsons: Vec<Entry>) -> Vec<Server> {
    jsons.iter().map( |entry| 
        json_to_server(&entry.value)
    ).collect()
}

fn server_to_entry(server: &Server) -> Entry {
    Entry {
        key: format!("{}", server.ipaddress),
        value: serde_json::to_string(server).unwrap()
    }    
}

pub async fn insert_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    inmemory::add_server(server);
    let result = persistence.insert(TABLE, server_to_entry(server)).await?;

    Ok(result > 0)
}

pub async fn update_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    inmemory::add_server(server);
    let result = persistence.update(TABLE, server_to_entry(server)).await?;

    Ok(result > 0)
}

pub async fn delete_server(persistence: &Persistence, ipaddress: &IpAddr) -> Result<bool, AppError> {
    inmemory::remove_server(ipaddress);
    let result = persistence.delete(TABLE, format!("{}", ipaddress).as_str()).await?;

    Ok(result > 0)
}



pub async fn load_all_servers(persistence: &Persistence, use_cache: bool) -> Result<Vec<Server>, AppError> {
    if use_cache {
        Ok(inmemory::get_all_servers())
    }
    else {
        let server_entries = persistence.get_all(TABLE, Some("inet_aton(key) asc")).await?;

        Ok(entries_to_servers(server_entries))
    }
}

pub async fn get_server(persistence: &Persistence, ipaddress: &IpAddr)  -> Result<Server, AppError> {
    let opt = persistence.get(TABLE, format!("{}", ipaddress).as_str()).await?;
    match opt {
        Some(entry) => {
            Ok(json_to_server(&entry.value))
        },
        None => {
            Err(AppError::ServerNotFound(format!("{}", ipaddress)))
        }
    }
}