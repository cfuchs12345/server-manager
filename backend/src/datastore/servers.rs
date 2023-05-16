
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
        key: server.ipaddress.clone(),
        value: serde_json::to_string(server).unwrap()
    }    
}

pub async fn insert_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    inmemory::add_server(server);
    let result = persistence.insert(TABLE, server_to_entry(server)).await.unwrap();

    Ok(result > 0)
}

pub async fn update_server(persistence: &Persistence, server: &Server) -> Result<bool, AppError> {
    inmemory::add_server(server);
    let result = persistence.update(TABLE, server_to_entry(server)).await.unwrap();

    Ok(result > 0)
}

pub async fn delete_server(persistence: &Persistence, ipaddress: &str) -> Result<bool, AppError> {
    inmemory::remove_server(ipaddress);
    let result = persistence.delete(TABLE, ipaddress).await.unwrap();

    Ok(result > 0)
}



pub async fn load_all_servers(persistence: &Persistence, use_cache: bool) -> Result<Vec<Server>, AppError> {
    if use_cache {
        Ok(inmemory::get_all_servers())
    }
    else {
        let server_entries = persistence.get_all(TABLE).await.unwrap();

        Ok(entries_to_servers(server_entries))
    }
}

pub async fn get_server(persistence: &Persistence, ipaddress: String)  -> Result<Server, AppError> {
    let opt = persistence.get(TABLE, &ipaddress).await.unwrap();
    match opt {
        Some(entry) => {
            Ok(json_to_server(&entry.value))
        },
        None => {
            Err(AppError::ServerNotFound(ipaddress))
        }
    }
}