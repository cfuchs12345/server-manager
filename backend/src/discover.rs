use dnsclient::{r#async::DNSClient, reexports::async_std::stream::StreamExt, UpstreamServer};
use futures::future::join_all;
use ipnet::Ipv4Net;
use lazy_static::lazy_static;
use log::{error, warn};
use reqwest::Url;

use std::{
    io::ErrorKind,
    net::{IpAddr, SocketAddrV4},
    time::Duration,
    vec,
};
use surge_ping::{Client, Config};
use tokio::sync::Semaphore;
use upnp_client::discovery;

use crate::{    
    config_types::DNSServer,
    plugin_types::{DetectionEntry, Plugin},
    plugins,
    server_types::{Feature, FeaturesOfServer, Server, Param},
    status,
    types::HostInformation,
};

lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);
}

pub async fn auto_discover_servers_in_network(
    network_as_string: &String,
    lookup_names: bool,
    dns_servers: Vec<DNSServer>,
) -> Result<Vec<HostInformation>, std::io::Error> {
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
            return Err(std::io::Error::from(ErrorKind::InvalidData));
        }
    };

    drop(permit);
    result
}

pub async fn discover_features_of_all_servers(
    servers: Vec<Server>,
    accept_self_signed_certificates: bool,
    plugin_base_path: String,
) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let dur = Duration::from_millis(30000);

    let mut features_from_upnp_discovery =
        match tokio::time::timeout(std::time::Duration::from_secs(20), do_upnp_discovery()).await {
            Ok(devices) => devices,
            Err(err) => Vec::new(),
        };

    // list of async tasks executed by tokio
    let mut tasks = Vec::new();
    for server in servers {
        let ipaddress = server.ipaddress.clone();
        let accept_self_signed_certificates = accept_self_signed_certificates;
        let plugin_base_path = plugin_base_path.clone();

        tasks.push(tokio::spawn(async move {
            discover_features(
                ipaddress.as_str(),
                accept_self_signed_certificates,
                &plugin_base_path,
            )
            .await
        }));
    }

    // wait for all tasks to finish
    let result = join_all(tasks).await;

    let mut features_from_plugin_discovery: Vec<FeaturesOfServer> = result
        .iter()
        .map(move |r| r.as_ref().unwrap())
        .map(move |r| r.as_ref().unwrap().to_owned())
        .filter(|f| !f.features.is_empty())
        .collect();

    Ok(merge_features(
        &mut features_from_plugin_discovery,
        &mut features_from_upnp_discovery,
    ))
}

async fn do_upnp_discovery() -> Vec<FeaturesOfServer> {
    let mut features_of_server = Vec::new();

    match upnp_client::discovery::discover_pnp_locations().await {
        Ok(devices) => {
            tokio::pin!(devices);

            while let Some(device) = devices.next().await {
                let mut features: Vec<Feature> = Vec::new();

                let json = serde_json::to_string_pretty(&device).unwrap();
                println!("{}", json);

                let mut params = Vec::new();
                params.push(Param {
                    name: "device_type".to_string(),
                    value: device.device_type,
                });
                params.push(Param {
                    name: "manufacturer".to_string(),
                    value: device.manufacturer,
                });
                params.push(Param {
                    name: "friendly_name".to_string(),
                    value: device.friendly_name,
                });
                params.push(Param {
                    name: "model_name".to_string(),
                    value: device.model_name,
                });
                params.push(Param {
                    name: "udn".to_string(),
                    value: device.udn,
                });
                if let Some(desc) = device.model_description {
                    params.push(Param {
                        name: "model_description".to_string(),
                        value: desc,
                    });
                }

                for service in device.services {
                    params.push(Param {
                        name: format!("service.{}.control_url", service.service_id),
                        value: service.control_url,
                    });
                    params.push(Param {
                        name: format!("service.{}.event_sub_url", service.service_id),
                        value: service.event_sub_url,
                    });
                    params.push(Param {
                        name: format!("service.{}.scpd_url", service.service_id),
                        value: service.scpd_url,
                    });
                    params.push(Param {
                        name: format!("service.{}.service_type", service.service_id),
                        value: service.service_type,
                    });

                    for action in service.actions {
                        params.push(Param {
                            name: format!(
                                "service.{}.{}.control_url",
                                service.service_id, action.name
                            ),
                            value: action.name.clone(),
                        });
                        for arg in action.arguments {
                            params.push(Param {
                                name: format!(
                                    "service.{}.{}.{}.related_state_variable",
                                    service.service_id, &action.name, arg.name
                                ),
                                value: arg.related_state_variable,
                            });
                            params.push(Param {
                                name: format!(
                                    "service.{}.{}.{}.direction",
                                    service.service_id, action.name, arg.name
                                ),
                                value: arg.direction,
                            });
                        }
                    }
                }

                features.push(Feature {
                    credentials: Vec::new(),
                    id: "upnp".to_string(),
                    name: "upnp".to_string(),
                    params: params,
                });

                match Url::parse(device.location.as_str()) {
                    Ok(url) => match url.host_str() {
                        Some(host) => {
                            features_of_server.push(FeaturesOfServer {
                                ipaddress: host.to_string(),
                                features,
                            });
                        }
                        None => {
                            log::error!("Could not find host information of url {}", url);
                        }
                    },
                    Err(err) => {
                        log::error!(
                            "Could not parse UPNP device location {} as URL. Error was: {}",
                            device.location,
                            err
                        );
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Error during UPNP discovery: {}", err);
            if err.to_string().contains("os error 10065") {
                log::error!("Error with code 10065 is most likely a firewall issue. UPnP results are not available")
            }
        }
    }

    features_of_server
}

pub async fn discover_features(
    ipaddress: &str,
    accept_self_signed_certificates: bool,
    plugin_base_path: &str,
) -> Result<FeaturesOfServer, std::io::Error> {
    let plugins = plugins::get_all_plugins(plugin_base_path).await.unwrap();

    let mut features_of_server = FeaturesOfServer {
        ipaddress: ipaddress.to_string(),
        features: vec![],
    };

    let client = create_http_client(accept_self_signed_certificates);

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
                                if check_plugin_match(&text, plugin.clone()).await {
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
) -> Result<Vec<HostInformation>, std::io::Error> {
    let socket_addresses = parse_ip_and_port_into_socket_address(dns_servers);

    let hosts = network.hosts();

    if hosts.count() > 512 {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
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
    list1: &mut Vec<FeaturesOfServer>,
    list2: &mut Vec<FeaturesOfServer>,
) -> Vec<FeaturesOfServer> {
    let mut result: Vec<FeaturesOfServer> = Vec::new();
    result.append(list1);
    for feature_list2 in list2 {
        if let Some(feature_list1) = result
            .iter()
            .find(|r| r.ipaddress == feature_list2.ipaddress)
        {
            feature_list1.to_owned().features.append(&mut feature_list2.features);
        } else {
            result.push(feature_list2.to_owned());
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
    dns_server.ipaddress.to_owned() + ":" + dns_server.port.to_string().as_str()
}

async fn discover_host(
    addr: IpAddr,
    client_v4: Client,
    lookup_names: bool,
    upsstream_server: Vec<UpstreamServer>,
) -> HostInformation {
    let ping_response = status::ping(addr, client_v4);

    let is_running = ping_response.await;

    let dnsnames = match lookup_names {
        true => lookup_hostname(addr, upsstream_server).await,
        false => vec![],
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

fn create_http_client(accept_self_signed_certificates: bool) -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
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

async fn check_plugin_match(input: &str, plugin: Plugin) -> bool {
    match plugins::plugin_detect_match(&plugin, input) {
        Ok(res) => res,
        Err(err) => {
            error!("{:?}", err);
            false
        }
    }
}
