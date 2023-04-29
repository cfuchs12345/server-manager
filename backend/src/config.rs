use crate::{config_types::DNSServer, persistence::{Entry, Persistence}};

const TABLE_DNS_SERVERS: &str = "dns_servers";

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


pub async fn save_dnsserver(persistence: &Persistence, server: &DNSServer) -> Result<bool, std::io::Error> {
    let result = persistence.insert(TABLE_DNS_SERVERS, dnsserver_to_entry(server)).await.unwrap();

    Ok(result > 0)
}


pub async fn delete_dnsserver(persistence: &Persistence, ipaddress: &str) -> Result<bool, std::io::Error> {
    let result = persistence.delete(TABLE_DNS_SERVERS, ipaddress).await.unwrap();

    Ok(result > 0)
}


pub async fn load_all_dnsservers(persistence: &Persistence) -> Result<Vec<DNSServer>,  std::io::Error> {
    let server_entries = persistence.get_all(TABLE_DNS_SERVERS).await.unwrap();


    Ok(entries_to_dnsservers(server_entries))
}