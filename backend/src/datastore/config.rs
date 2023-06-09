use std::net::IpAddr;

use crate::{
    common,
    models::{config::dns_server::DNSServer, error::AppError},
};

use super::{persistence::Persistence, Entry};

const TABLE_DNS_SERVERS: &str = "dns_servers";
const TABLE_ENCRYPTION: &str = "encryption";

fn json_to_dnsserver(json: &str) -> Result<DNSServer, AppError> {
    serde_json::from_str(json).map_err(AppError::from)
}

fn entries_to_dnsservers(jsons: Vec<Entry>) -> Result<Vec<DNSServer>, AppError> {
    let mut dns_servers = Vec::new();
    for entry in jsons {
        dns_servers.push(json_to_dnsserver(&entry.value)?);
    }
    Ok(dns_servers)
}

fn dnsserver_to_entry(dns_server: &DNSServer) -> Result<Entry, AppError> {
    Ok(Entry {
        key: format!("{}", dns_server.ipaddress),
        value: serde_json::to_string(dns_server)?,
    })
}

pub async fn insert_dnsserver(
    persistence: &Persistence,
    server: &DNSServer,
) -> Result<bool, AppError> {
    let result = persistence
        .insert(TABLE_DNS_SERVERS, dnsserver_to_entry(server)?)
        .await?;

    Ok(result > 0)
}

pub async fn delete_dnsserver(
    persistence: &Persistence,
    ipaddress: &IpAddr,
) -> Result<bool, AppError> {
    let result = persistence
        .delete(TABLE_DNS_SERVERS, format!("{}", ipaddress).as_str())
        .await?;

    Ok(result > 0)
}

pub async fn load_all_dnsservers(persistence: &Persistence) -> Result<Vec<DNSServer>, AppError> {
    let server_entries = persistence
        .get_all(TABLE_DNS_SERVERS, Some("inet_aton(key) asc"))
        .await?;

    entries_to_dnsservers(server_entries)
}

pub async fn insert_new_encryption_key(persistence: &Persistence) -> Result<u64, AppError> {
    persistence
        .insert(
            TABLE_ENCRYPTION,
            Entry {
                key: "default".to_string(),
                value: common::get_random_key32()?,
            },
        )
        .await
        .map_err(AppError::from)
}
