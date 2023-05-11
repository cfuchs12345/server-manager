use std::{path::Path, net::{IpAddr}};

use crate::{config_types::DNSServer, inmemory};
use sys_info;

pub fn get_memory_usage() -> String {
    match sys_info::mem_info() {
        Ok(mem) => {
            format!("mem: total {} KB, free {} KB, avail {} KB",
            mem.total, mem.free, mem.avail)
        },
        Err(err) => {
            format!("Could not get memory information. Error was: {}", err)
        }
    }
}

pub fn get_load_info() -> String {
    match sys_info::loadavg() {
        Ok(load) => {
            format!("load: {} {} {}", load.one, load.five, load.fifteen)
        },
        Err(err) => {
            format!("Could not get load information. Error was: {}", err)
        }
    }
}

pub fn get_systenms_dns_servers() -> Vec<DNSServer> {
    let mut list = Vec::new();
    let path = Path::new("/etc/resolv.conf"); // path for debian linux
    
    if path.exists() && path.is_file() {        
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let mut found = get_private_dns_servers_from_config(content);
                

                list.append(&mut found);
            },
            Err(err) => {
                log::error!("Could not read file {:?} Error was: {}", path, err);
            }
        }
    }
    else {
        if let Ok(dev_server) = inmemory::get_config().get("dev_dns_server1") {
            list.push(DNSServer { ipaddress: dev_server, port: 53 })
        }
        if let Ok(dev_server) = inmemory::get_config().get("dev_dns_server2") {
            list.push(DNSServer { ipaddress: dev_server, port: 53 })
        }

        if list.is_empty() {
            log::warn!("No known location for DNS configuration. Cannot get system DNS servers automatically");
        }
    }
    list
}

fn get_private_dns_servers_from_config(config_file_content: String)  -> Vec<DNSServer> {
    config_file_content.split('\n').filter_map(get_dns_from_line).collect()
}

fn get_dns_from_line(line: &str) -> Option<DNSServer> {
    if line.starts_with("nameserver ") {
        let ip = line.replace("nameserver ", "");

        let address:Result<IpAddr,_> = ip.parse();

        match address {
            Ok(an_address) => {
                match an_address {
                    IpAddr::V4(ipv4) => {
                        if ipv4.is_private() {
                            return Some(DNSServer{
                                ipaddress: ip,
                                port: 53 // resolv.conf is not allowing ports as far as I know, so it should be always 53
                            });
                        }
                    },
                    IpAddr::V6(ipv6) => {
                        let v6_upper = ipv6.to_string().to_uppercase();
                        if v6_upper.starts_with("FE80") || v6_upper.starts_with("FC00") { // currently only link local addresses or RFC 4193 ULA. Hard to tell if it is a private address othewise, if a prefix is used
                            return Some(DNSServer{
                                ipaddress: ipv6.to_string(),
                                port: 53 // resolv.conf is not allowing ports as far as I know, so it should be always 53
                            });
                        }
                    }
                }               
            }
            Err(err) => {
                log::error!("Could not parse address {} due to error {}", ip, err);
            }
        }
        
    }

    None
}