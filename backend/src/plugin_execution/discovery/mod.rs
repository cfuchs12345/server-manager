use dnsclient::{r#async::DNSClient, UpstreamServer};
use futures::future::join_all;
use ipnet::Ipv4Net;
use lazy_static::lazy_static;
use log::error;

use local_ip_address::list_afinet_netifas;
use std::{
    cmp::Ordering,
    net::{IpAddr, SocketAddr},
    vec,
};
use tokio::sync::Semaphore;

use crate::{
    commands::{
        self, http::HttpCommandResult, ping::PingCommandResult, socket::SocketCommandResult,
    },
    common, datastore,
    models::{
        config::dns_server::DNSServer, error::AppError, response::host_information::HostInformation,
    },
    models::{
        plugin::Plugin,
        server::{Feature, FeaturesOfServer, Server},
    },
    other_functions::upnp,
};

lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);
    static ref LOCAL_IP_ADDRESSES: Vec<IpAddr> = get_local_addresses();
}

pub async fn auto_discover_servers_in_network(
    network_as_string: &String,
    lookup_names: bool,
    dns_servers: Vec<DNSServer>,
    silent: &bool,
) -> Result<Vec<HostInformation>, AppError> {
    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await?;

    log::debug!(
        "called auto discover with network param {:?}",
        network_as_string
    );

    let result = match network_as_string.parse() {
        Ok(parsed_network) => {
            auto_discover_servers(&parsed_network, lookup_names, dns_servers, silent).await
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
    silent: &bool,
) -> Result<Vec<FeaturesOfServer>, AppError> {
    let wait_time_for_upnp = 15; // in seconds

    let upnp_future = upnp::upnp_discover(wait_time_for_upnp, upnp_activated);
    let crypto_key = datastore::get_crypto_key()?;

    // list of async tasks executed by tokio
    let mut tasks = Vec::new();
    for server in servers.clone() {
        let crypto_key = crypto_key.clone();
        let silent = *silent;

        tasks.push(tokio::spawn(async move {
            let server = server;
            discover_features(server.get_ipaddress(), crypto_key, &silent).await
        }));
    }

    // wait for all tasks to finish
    let result = join_all(tasks).await;

    let features_from_plugin_discovery: Vec<FeaturesOfServer> = result
        .iter()
        .flat_map(|f| f.as_ref().expect("Could not get ref"))
        .map(|f| f.to_owned())
        .collect();

    let features_from_upnp_discovery = upnp_future.await?;
    log::debug!(
        "features_from_upnp_discovery {:?}",
        features_from_upnp_discovery
    );

    let all_found = merge_features(features_from_plugin_discovery, features_from_upnp_discovery);

    let filtered_only_new = filter(&all_found, &servers);

    Ok(filtered_only_new)
}

pub async fn discover_features(
    ipaddress: IpAddr,
    crypto_key: String,
    silent: &bool,
) -> Result<FeaturesOfServer, AppError> {
    let mut features_of_server = FeaturesOfServer {
        ipaddress,
        features: vec![],
    };

    let plugins = crate::datastore::get_all_plugins()?;

    for plugin in plugins {
        if !plugin.detection.detection_possible {
            log::debug!("Skipping plugin {}", plugin.id);
            continue;
        }

        'outer: for detection_entry in &plugin.detection.list {
            let url = detection_entry.args.iter().find(|a| a.name == "url");

            log::debug!("current entry {:?}", url);

            log::debug!("checking url {:?} for plugin {}", &url, &plugin.name);

            let response = match plugin.detection.command.as_str() {
                commands::socket::SOCKET => {
                    // Socket makes only sense for the server where the manager itself is runnin
                    if ipaddress.is_loopback()
                        || LOCAL_IP_ADDRESSES
                            .iter()
                            .any(|a| a.cmp(&ipaddress) == Ordering::Equal)
                    {
                        log::debug!(
                            "Trying to discover via socket for server {} {:?}",
                            ipaddress,
                            plugin
                        );

                        let Ok(inputs) =
                            commands::socket::make_command_input_from_detection(ipaddress, crypto_key.as_str(), &plugin, detection_entry, silent).await else {
                                log::error!("Could not create command input for {:?}", detection_entry);
                                return Err(AppError::Unknown("Could not create command input".to_owned()));
                            };

                        let mut response: Option<String> = None;

                        for input in inputs {
                            response = match commands::execute::<SocketCommandResult>(input, silent)
                                .await
                            {
                                Ok(result) => {
                                    log::debug!("result is {:?}", result);

                                    Some(result.get_response())
                                }
                                Err(err) => {
                                    err.log();
                                    continue;
                                }
                            }
                        }
                        response
                    } else {
                        log::debug!(
                            "Socket connection only available for loopback or local ip address"
                        );
                        None
                    }
                }
                _ => {
                    let Ok(inputs) = commands::http::make_command_input_from_detection(
                        &ipaddress,
                        crypto_key.as_str(),
                        &plugin,
                        detection_entry,
                        silent
                    ).await else {
                        log::error!("Could not create command input for {:?}", detection_entry);
                        return Err(AppError::Unknown("Could not create command input".to_owned()));
                    };

                    let mut response: Option<String> = None;

                    for input in inputs {
                        response = match commands::execute::<HttpCommandResult>(input, silent).await
                        {
                            Ok(result) => Some(result.get_response()),
                            Err(err) => {
                                err.log();
                                continue;
                            }
                        }
                    }
                    response
                }
            };

            log::debug!(
                "Response from detection: {:?} {:?} {}",
                response,
                plugin,
                ipaddress
            );

            if check_plugin_match(&response, &plugin).await {
                log::debug!(
                    "Plugin {:?} matched for server {} for response {:?}",
                    &plugin.id,
                    ipaddress,
                    response
                );

                features_of_server
                    .features
                    .push(create_feature_from_plugin(&plugin));

                break 'outer; // early exit of both loops if we found a match
            } else {
                log::debug!(
                    "Connect successful but content\n {:?}\n did not match with script {:?}",
                    response,
                    plugin.detection.script.script
                );
            }
        }
    }

    Ok(features_of_server)
}

fn get_local_addresses() -> Vec<IpAddr> {
    let network_interfaces = list_afinet_netifas();
    let mut vec = Vec::new();

    if let Ok(network_interfaces) = network_interfaces {
        for (_name, ip) in network_interfaces.iter() {
            vec.push(ip.to_owned());
        }
    }
    log::debug!("Local IP addresses: {:?}", vec);
    vec
}

async fn auto_discover_servers(
    network: &Ipv4Net,
    lookup_names: bool,
    dns_servers: Vec<DNSServer>,
    silent: &bool,
) -> Result<Vec<HostInformation>, AppError> {
    let socket_addresses = parse_ip_and_port_into_socket_address(dns_servers)?;

    let hosts = network.hosts();

    if hosts.count() > 512 {
        return Err(AppError::InvalidArgument(
            "Too many hosts in the network".to_owned(),
            None,
        ));
    }

    // list of async tasks executed by tokio
    let mut tasks = Vec::new();

    for ipv4_addr in hosts {
        let addr = IpAddr::V4(ipv4_addr);

        let upstream_servers = socket_addresses
            .iter()
            .map(|socket_addr| UpstreamServer::new(*socket_addr))
            .collect();

        let silent = *silent;

        tasks.push(tokio::spawn(async move {
            discover_host(addr, lookup_names, upstream_servers, &silent).await
        }));
    }
    // wait for all tasks to finish
    let result = join_all(tasks).await;

    let list = result
        .iter()
        .flat_map(move |r| r.as_ref().ok())
        .flat_map(|r| r.as_ref().ok())
        .map(|hi| hi.to_owned())
        .collect();

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
        if let Some(to_add) = list2
            .iter()
            .find(|f| f.ipaddress == feature.ipaddress)
            .cloned()
            .as_mut()
        {
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

fn parse_ip_and_port_into_socket_address(
    dns_servers: Vec<DNSServer>,
) -> Result<Vec<SocketAddr>, AppError> {
    let mut list = Vec::new();
    for dns_server in dns_servers {
        let socket_addr: SocketAddr = concat_ip_and_port(&dns_server).parse()?;

        list.push(socket_addr);
    }
    Ok(list)
}

fn concat_ip_and_port(dns_server: &DNSServer) -> String {
    format!("{}:{}", dns_server.ipaddress, dns_server.port)
}

async fn discover_host(
    addr: IpAddr,
    lookup_names: bool,
    upsstream_server: Vec<UpstreamServer>,
    silent: &bool,
) -> Result<HostInformation, AppError> {
    let input = commands::ping::make_input(addr);
    let ping_response_fut = commands::execute::<PingCommandResult>(input, silent);

    let lookup_hostname_fut = match lookup_names {
        true => Some(lookup_hostname(addr, upsstream_server)),
        false => None,
    };

    let result = ping_response_fut.await?;
    let dnsnames = match lookup_hostname_fut {
        Some(servers) => servers.await?,
        None => Vec::new(),
    };

    Ok(HostInformation {
        is_running: result.get_result(),
        ipaddress: addr,
        dnsname: dnsnames.join(","),
    })
}

async fn lookup_hostname(
    addr: IpAddr,
    upsstream_server: Vec<UpstreamServer>,
) -> Result<Vec<String>, AppError> {
    let client = DNSClient::new(upsstream_server);
    Ok(client.query_ptr(&addr).await?)
}

fn create_feature_from_plugin(plugin: &Plugin) -> Feature {
    Feature {
        id: plugin.id.clone(),
        name: plugin.name.clone(),
        params: vec![],
        credentials: vec![],
    }
}

async fn check_plugin_match(input: &Option<String>, plugin: &Plugin) -> bool {
    let Some(input_to_check) = input else {
        return false;
    };

    match common::script_match(&plugin.detection.script, input_to_check.as_str()) {
        Ok(res) => res,
        Err(err) => {
            error!("{:?}", err);
            false
        }
    }
}

fn filter(all_found: &[FeaturesOfServer], servers: &[Server]) -> Vec<FeaturesOfServer> {
    all_found
        .iter()
        .map(|found_features_for_server| filter_features(found_features_for_server, servers)) // first update features list and remove single features
        .filter(|found_features_for_server| {
            // filter out FeaturesOfServer where the list of features is empty after the step before (when there is no new feature for a server and all are already known)
            !found_features_for_server.features.is_empty()
        })
        .collect()
}

fn filter_features(
    found_features_for_server: &FeaturesOfServer,
    servers: &[Server],
) -> FeaturesOfServer {
    let mut updated = found_features_for_server.to_owned();

    if let Some(server) = servers
        .iter()
        .find(|server| server.get_ipaddress() == found_features_for_server.ipaddress)
    {
        let only_new_features: Vec<Feature> = updated
            .features
            .iter()
            .filter(|found_feature| !server_has_feature(found_feature, server))
            .map(|f| f.to_owned())
            .collect();

        updated.features = only_new_features;
    }
    updated
}

fn server_has_feature(found_feature: &Feature, server: &Server) -> bool {
    server
        .get_features()
        .iter()
        .any(|existing_feature| existing_feature.id == found_feature.id)
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
        let i = Some(input.to_owned());
        let result = check_plugin_match(&i, &plugin.expect("should not happen")).await;

        assert!(result);
    }

    #[test]
    fn test_merge_features() {
        let list1 = vec![
            FeaturesOfServer {
                ipaddress: "192.168.178.1".parse().expect("should not happen"),
                features: vec![Feature {
                    id: "proxmox".to_string(),
                    name: "proxmox".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
            FeaturesOfServer {
                ipaddress: "192.168.178.2".parse().expect("should not happen"),
                features: vec![Feature {
                    id: "nas".to_string(),
                    name: "nas".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
        ];

        let list2 = vec![FeaturesOfServer {
            ipaddress: "192.168.178.2".parse().expect("should not happen"),
            features: vec![Feature {
                id: "upnp".to_string(),
                name: "upnp".to_string(),
                credentials: vec![],
                params: vec![],
            }],
        }];

        let res = merge_features(list1, list2);

        assert_eq!(res.len(), 2); // since only two different ips
        assert_eq!(res.get(0).expect("should not happen").features.len(), 1); // only proxmox
        assert_eq!(res.get(1).expect("should not happen").features.len(), 2); // nas and upnp
    }

    #[test]
    fn test_merge_features2() {
        let list1 = vec![
            FeaturesOfServer {
                ipaddress: "192.168.178.1".parse().expect("should not happen"),
                features: vec![Feature {
                    id: "proxmox".to_string(),
                    name: "proxmox".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
            FeaturesOfServer {
                ipaddress: "192.168.178.2".parse().expect("should not happen"),
                features: vec![Feature {
                    id: "nas".to_string(),
                    name: "nas".to_string(),
                    credentials: vec![],
                    params: vec![],
                }],
            },
        ];

        let list2 = vec![FeaturesOfServer {
            ipaddress: "192.168.178.3".parse().expect("should not happen"),
            features: vec![Feature {
                id: "upnp".to_string(),
                name: "upnp".to_string(),
                credentials: vec![],
                params: vec![],
            }],
        }];

        let res = merge_features(list1, list2);

        assert_eq!(res.len(), 3); // since only two different ips
        assert_eq!(res.get(0).expect("should not happen").features.len(), 1); // only proxmox
        assert_eq!(res.get(1).expect("should not happen").features.len(), 1); // nas
        assert_eq!(res.get(2).expect("should not happen").features.len(), 1); // upnp
    }
}
