use dnsclient::{r#async::DNSClient, UpstreamServer};
use futures::future::join_all;
use ipnet::Ipv4Net;
use lazy_static::lazy_static;
use log::{error, warn};

use std::{
    net::{IpAddr, SocketAddrV4},
    vec,
};
use surge_ping::{Client, Config};
use tokio::sync::Semaphore;

use crate::{
    commands::{self},
    common,
    models::{
        config::dns_server::DNSServer, error::AppError, response::host_information::HostInformation,
    },
    models::{
        plugin::{detection::DetectionEntry, Plugin},
        server::{Feature, FeaturesOfServer, Server},
    },
    other_functions::upnp,
};

lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);
}

pub async fn auto_discover_servers_in_network(
    network_as_string: &String,
    lookup_names: bool,
    dns_servers: Vec<DNSServer>,
) -> Result<Vec<HostInformation>, AppError> {
    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await.unwrap();

    log::debug!(
        "called auto discover with network param {:?}",
        network_as_string
    );

    let result = match network_as_string.parse() {
        Ok(parsed_network) => {
            auto_discover_servers(&parsed_network, lookup_names, dns_servers).await
        }
        e => {
            error!(
                "Could not parse network {:?}. Error was {:?}",
                network_as_string, e
            );
            return Err(AppError::InvalidArgument(
                "network".to_owned(),
                Some(network_as_string.to_owned()),
            ));
        }
    };

    drop(permit);
    result
}

pub async fn discover_features_of_all_servers(
    servers: Vec<Server>,
    upnp_activated: bool,
) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let wait_time_for_upnp = 15; // in seconds

    let upnp_future = upnp::upnp_discover(wait_time_for_upnp, upnp_activated);

    // list of async tasks executed by tokio
    let mut tasks = Vec::new();
    for server in servers {
        let ip_address = server.ipaddress.clone();

        tasks.push(tokio::spawn(async move {
            discover_features(ip_address.as_str()).await
        }));
    }

    // wait for all tasks to finish
    let result = join_all(tasks).await;

    let features_from_plugin_discovery: Vec<FeaturesOfServer> = result
        .iter()
        .map(|f| f.as_ref().unwrap())
        .map(|f| f.as_ref().unwrap())
        .map(|f| f.to_owned())
        .collect();

    let features_from_upnp_discovery = upnp_future.await?;
    log::info!(
        "features_from_upnp_discovery {:?}",
        features_from_upnp_discovery
    );

    Ok(merge_features(
        features_from_plugin_discovery,
        features_from_upnp_discovery,
    ))
}

pub async fn discover_features(ipaddress: &str) -> Result<FeaturesOfServer, std::io::Error> {
    let mut features_of_server = FeaturesOfServer {
        ipaddress: ipaddress.to_string(),
        features: vec![],
    };

    let client = common::create_http_client();

    let plugins = crate::datastore::get_all_plugins();

    for plugin in plugins {
        if !plugin.detection.detection_possible {
            log::debug!("Skipping plugin {}", plugin.id);
            continue;
        }

        'outer: for detection_entry in &plugin.detection.list {
            log::debug!("current entry {}", detection_entry.url);

            for url in get_urls_for_check(detection_entry, ipaddress) {
                log::debug!("checking url {} for plugin {}", &url, &plugin.name);

                match client.get(&url).send().await {
                    Ok(body) => {
                        match body.text().await {
                            Ok(text) => {
                                if check_plugin_match(&text, &plugin).await {
                                    log::debug!(
                                        "Plugin {:?} matched for server {}",
                                        &plugin.id,
                                        ipaddress
                                    );

                                    features_of_server
                                        .features
                                        .push(create_feature_from_plugin(&plugin));

                                    break 'outer; // early exit of both loops if we found a match
                                } else {
                                    log::debug!("Connect successful but content\n {:?}\n did not match with script {:?}", text, plugin.detection.script.script);
                                }
                            }
                            Err(err) => {
                                error!(
                                    "Unexpected error while checking result of url {} {:?}",
                                    url, err
                                );
                            }
                        }
                    }
                    Err(err) => {
                        warn!("{:?}", err);
                        continue;
                    }
                };
            }
        }
    }

    Ok(features_of_server)
}

async fn auto_discover_servers(
    network: &Ipv4Net,
    lookup_names: bool,
    dns_servers: Vec<DNSServer>,
) -> Result<Vec<HostInformation>, AppError> {
    let socket_addresses = parse_ip_and_port_into_socket_address(dns_servers);

    let hosts = network.hosts();

    if hosts.count() > 512 {
        return Err(AppError::InvalidArgument(
            "Too many hosts in the network".to_owned(),
            None,
        ));
    }

    let list = match Client::new(&Config::default()) {
        Ok(client) => {
            // list of async tasks executed by tokio
            let mut tasks = Vec::new();

            for ipv4_addr in hosts {
                let addr = IpAddr::V4(ipv4_addr);

                let upstream_servers = socket_addresses
                    .iter()
                    .map(|socket_addr| UpstreamServer::new(*socket_addr))
                    .collect();

                tasks.push(tokio::spawn(discover_host(
                    addr,
                    client.clone(),
                    lookup_names,
                    upstream_servers,
                )));
            }
            // wait for all tasks to finish
            let result = join_all(tasks).await;

            result
                .iter()
                .map(move |r| r.as_ref().unwrap().to_owned())
                .collect()
        }
        e => {
            error!("Error while creating ping client: {:?}", e.err());
            vec![]
        }
    };

    Ok(list)
}

fn merge_features(
    mut list1: Vec<FeaturesOfServer>,
    list2: Vec<FeaturesOfServer>,
) -> Vec<FeaturesOfServer> {
    let mut result: Vec<FeaturesOfServer> = Vec::new();
    result.append(&mut list1);

    // test if we need to merge with an entry that is already there for a server and we just need to add features to the list
    for feature in &mut result {
        if let Some(to_add) = list2.iter().find( |f| f.ipaddress == feature.ipaddress).cloned().as_mut() {            
            feature.features.append(&mut to_add.features);            
        }
    }

    // if server is not already there, just add the whole FeaturesOfServer
    for feature in list2 {
        if !result.iter().any(|f| f.ipaddress == feature.ipaddress) {
            result.push(feature);
        }
    }

    result
}

fn parse_ip_and_port_into_socket_address(dns_servers: Vec<DNSServer>) -> Vec<SocketAddrV4> {
    dns_servers
        .iter()
        .map(|dns_server| {
            let socket_addr: SocketAddrV4 = concat_ip_and_port(dns_server).parse().unwrap();
            socket_addr
        })
        .collect()
}

fn concat_ip_and_port(dns_server: &DNSServer) -> String {
    format!("{}:{}", dns_server.ipaddress, dns_server.port)
}

async fn discover_host(
    addr: IpAddr,
    client_v4: Client,
    lookup_names: bool,
    upsstream_server: Vec<UpstreamServer>,
) -> HostInformation {
    let ping_response_fut = commands::ping::ping(addr, client_v4);

    let lookup_hostname_fut = match lookup_names {
        true => Some(lookup_hostname(addr, upsstream_server)),
        false => None,
    };

    let is_running = ping_response_fut.await;
    let dnsnames = match lookup_hostname_fut {
        Some(servers) => servers.await,
        None => Vec::new(),
    };

    HostInformation {
        is_running,
        ipaddress: addr.to_string(),
        dnsname: dnsnames.join(","),
    }
}

async fn lookup_hostname(addr: IpAddr, upsstream_server: Vec<UpstreamServer>) -> Vec<String> {
    let client = DNSClient::new(upsstream_server);
    let result = client.query_ptr(&addr).await;

    result.unwrap()
}

fn get_urls_for_check(detection_entry: &DetectionEntry, ipaddress: &str) -> Vec<String> {
    detection_entry
        .defaultports
        .iter()
        .map(|port| {
            let url = detection_entry
                .url
                .replace("${IP}", ipaddress)
                .replace("${PORT}", port.to_string().as_str());
            url
        })
        .collect()
}

fn create_feature_from_plugin(plugin: &Plugin) -> Feature {
    Feature {
        id: plugin.id.clone(),
        name: plugin.name.clone(),
        params: vec![],
        credentials: vec![],
    }
}

async fn check_plugin_match(input: &str, plugin: &Plugin) -> bool {
    match plugin_detect_match(plugin, input) {
        Ok(res) => res,
        Err(err) => {
            error!("{:?}", err);
            false
        }
    }
}

pub fn plugin_detect_match(plugin: &Plugin, input: &str) -> Result<bool, AppError> {
    let script = plugin.detection.script.script.clone();
    let script_type = plugin.detection.script.script_type.clone();

    let is_lua = matches!(script_type.as_str(), "lua");
    let is_rhai = matches!(script_type.as_str(), "rhai");

    if is_lua {
        Ok(common::match_with_lua(input, &script))
    } else if is_rhai {
        Ok(common::match_with_rhai(input, &script))
    } else {
        Err(AppError::InvalidArgument(
            "script".to_string(),
            Some(script_type),
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::datastore;

    use super::*;

    #[tokio::test]
    async fn test_match() {
        let input = "<result>\
                <application>sleep-on-lan</application>\
                <version>1.1.1-RELEASE</version>\
                <compilation-timestamp>2022-08-13T22:25:28+0200</compilation-timestamp>\
                <commit>35982e56d2bf98f27afb01a2cfc793754af8d3da</commit>\
                <hosts>\
                <host ip=\"127.0.0.1/8\" mac=\"\" reversed-mac=\"\"/>\
                <host ip=\"192.168.178.20/24\" mac=\"6c:4b:90:66:3b:91\" reversed-mac=\"91:3b:66:90:4b:6c\"/>\
                <host ip=\"192.168.179.2/24\" mac=\"00:00:00:00:1a:54\" reversed-mac=\"54:1a:00:00:00:00\"/>\
                <host ip=\"192.168.222.1/24\" mac=\"12:af:1a:8a:dc:96\" reversed-mac=\"96:dc:8a:1a:af:12\"/>\
                </hosts>\
                <listeners>\
                <listener type=\"UDP\" port=\"9\" active=\"true\"/>\
                <listener type=\"HTTP\" port=\"8009\" active=\"true\"/>\
                </listeners>\
                <commands>\
                <command operation=\"sleep\" command=\"systemctl suspend\" default=\"true\" type=\"external\"/>\
                <command operation=\"shutdown\" command=\"shutdown -h\" default=\"false\" type=\"external\"/>\
                </commands>\
                </result>";

        let plugin = datastore::load_plugin("shipped_plugins/plugins", "sleep.json").await;

        let result = plugin_detect_match(&plugin.unwrap(), input);

        assert_eq!(true, result.unwrap());
    }

    #[test]
    fn test_merge_features() {
        let list1 = vec![
            FeaturesOfServer {
                ipaddress: "192.168.178.1".to_owned(),
                features: vec![Feature {
                    id: "proxmox".to_string(),
                    name: "proxmox".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
            FeaturesOfServer {
                ipaddress: "192.168.178.2".to_owned(),
                features: vec![Feature {
                    id: "nas".to_string(),
                    name: "nas".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
        ];

        let list2 = vec![FeaturesOfServer {
            ipaddress: "192.168.178.2".to_owned(),
            features: vec![Feature {
                id: "upnp".to_string(),
                name: "upnp".to_string(),
                credentials: vec![],
                params: vec![],
            }],
        }];

        let res = merge_features(list1, list2);

        assert_eq!(res.len(), 2); // since only two different ips
        assert_eq!(res.get(0).unwrap().features.len(), 1); // only proxmox
        assert_eq!(res.get(1).unwrap().features.len(), 2); // nas and upnp
    }


    #[test]
    fn test_merge_features2() {
        let list1 = vec![
            FeaturesOfServer {
                ipaddress: "192.168.178.1".to_owned(),
                features: vec![Feature {
                    id: "proxmox".to_string(),
                    name: "proxmox".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
            FeaturesOfServer {
                ipaddress: "192.168.178.2".to_owned(),
                features: vec![Feature {
                    id: "nas".to_string(),
                    name: "nas".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
        ];

        let list2 = vec![FeaturesOfServer {
            ipaddress: "192.168.178.3".to_owned(),
            features: vec![Feature {
                id: "upnp".to_string(),
                name: "upnp".to_string(),
                credentials: vec![],
                params: vec![],
            }],
        }];

        let res = merge_features(list1, list2);

        assert_eq!(res.len(), 3); // since only two different ips
        assert_eq!(res.get(0).unwrap().features.len(), 1); // only proxmox
        assert_eq!(res.get(1).unwrap().features.len(), 1); // nas
        assert_eq!(res.get(2).unwrap().features.len(), 1); // upnp
    }
}
