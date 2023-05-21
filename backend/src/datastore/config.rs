use crate::{models::{config::dns_server::DNSServer, error::AppError}, common};

use super::{persistence::Persistence, Entry};



const TABLE_DNS_SERVERS: &str = "dns_servers";
const TABLE_ENCRYPTION: &str = "encryption";

fn json_to_dnsserver(json: &str) -> DNSServer {
    serde_json::from_str(json).unwrap()
}

fn entries_to_dnsservers(jsons: Vec<Entry>) -> Vec<DNSServer> {
    jsons.iter().map( |entry| 
        json_to_dnsserver(&entry.value)
    ).collect()
}

fn dnsserver_to_entry(server: &DNSServer) -> Entry {
    Entry {
        key: server.ipaddress.clone(),
        value: serde_json::to_string(server).unwrap()
    }    
}


pub async fn insert_dnsserver(persistence: &Persistence, server: &DNSServer) -> Result<bool, AppError> {
    let result = persistence.insert(TABLE_DNS_SERVERS, dnsserver_to_entry(server)).await.unwrap();

    Ok(result > 0)
}


pub async fn delete_dnsserver(persistence: &Persistence, ipaddress: &str) -> Result<bool, AppError> {
    let result = persistence.delete(TABLE_DNS_SERVERS, ipaddress).await.unwrap();

    Ok(result > 0)
}


pub async fn load_all_dnsservers(persistence: &Persistence) -> Result<Vec<DNSServer>, AppError> {
    let server_entries = persistence.get_all(TABLE_DNS_SERVERS, Some("inet_aton(key) asc")).await.unwrap();


    Ok(entries_to_dnsservers(server_entries))
}




pub async fn insert_new_encryption_key(persistence: &Persistence) {
    persistence
        .insert(
            TABLE_ENCRYPTION,
            Entry {
                key: "default".to_string(),
                value: common::get_random_key32().unwrap(),
            },
        )
        .await
        .unwrap();
}