use dnsclient::{r#async::DNSClient, UpstreamServer};
use futures::future::join_all;
use ipnet::Ipv4Net;
use lazy_static::lazy_static;
use log::{error, warn};
use rand::random;
use std::{
    io::ErrorKind,
    net::{IpAddr, SocketAddrV4},
    time::Duration,
    vec,
};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::{sync::Semaphore, time};

use crate::{
    plugins,
    types::{HostInformation, Status},
    plugin_types::{DetectionEntry, Plugin },
    server_types::{Feature, Server, FeaturesOfServer}, config_types::DNSServer
};

lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);
}

pub async fn status_check(ips_to_check: Vec<String>) -> Result<Vec<Status>, std::io::Error> {
    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await.unwrap();

    let list = match Client::new(&Config::default()) {
        Ok(client) => {
            // list of async tasks executed by tokio
            let mut tasks = Vec::new();

            for ip in ips_to_check {
                match ip.parse() {
                    Ok(ipaddress) => {
                        tasks.push(tokio::spawn(get_host_status(
                            IpAddr::V4(ipaddress),
                            client.clone(),
                        )));
                    }
                    Err(err) => {
                        error!("Error while parsing address {:?} was {:?}", ip, err);
                    }
                }
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

    drop(permit);
    Ok(list)
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
    // list of async tasks executed by tokio
    let mut tasks = Vec::new();
    for server in servers {
        let ipaddress = server.ipaddress.clone();
        let accept_self_signed_certificates = accept_self_signed_certificates.clone();
        let plugin_base_path = plugin_base_path.clone();

        tasks.push(tokio::spawn(async move {
            return discover_features(
                ipaddress.as_str(),
                &accept_self_signed_certificates,
                &plugin_base_path,
            )
            .await;
        }));
    }

    // wait for all tasks to finish
    let result = join_all(tasks).await;

    let res: Vec<FeaturesOfServer> = result
        .iter()
        .map(move |r| r.as_ref().unwrap())
        .map(move |r| r.as_ref().unwrap().to_owned())
        .filter(|f| f.features.len() > 0)
        .collect();

    Ok(res)
}

pub async fn discover_features(
    ipaddress: &str,
    accept_self_signed_certificates: &bool,
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

            for url in get_urls_for_check(&detection_entry, ipaddress) {
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

                let upstream_servers = socket_addresses.iter().map( |socket_addr| UpstreamServer::new(socket_addr.clone())).collect();

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
    let ping_response = ping(addr, client_v4);

    let is_running = ping_response.await;

    let dnsnames = match lookup_names {
        true => lookup_hostname(addr, upsstream_server).await,
        false => vec![],
    };

    let host_information = HostInformation {
        is_running: is_running,
        ipaddress: addr.to_string(),
        dnsname: dnsnames.join(","),
    };

    host_information
}

async fn lookup_hostname(addr: IpAddr, upsstream_server: Vec<UpstreamServer>) -> Vec<String> {
    let client = DNSClient::new(upsstream_server);
    let result = client.query_ptr(&addr).await;

    result.unwrap()
}

async fn get_host_status(addr: IpAddr, client: Client) -> Status {
    let result = ping(addr, client).await;

    Status {
        ipaddress: addr.to_string(),
        is_running: result,
    }
}

async fn ping(addr: IpAddr, client: Client) -> bool {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(1));

    let mut interval = time::interval(Duration::from_secs(1));
    let mut reachable = false;

    for idx in 0..3 {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {
            Ok((IcmpPacket::V4(_packet), _dur)) => {
                reachable = true;
                break;
            }
            Ok((IcmpPacket::V6(_packet), _dur)) => {
                reachable = true;
                break;
            }
            Err(_e) => {
                reachable = false;
            }
        };
    }
    reachable
}



fn create_http_client(accept_self_signed_certificates: &bool) -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_self_signed_certificates.clone())
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
        ..Default::default()
    }
}

async fn check_plugin_match(input: &str, plugin: Plugin) -> bool {
    match plugins::plugin_detect_match(&plugin, &input) {
        Ok(res) => {
            return res;
        }
        Err(err) => {
            error!("{:?}", err);
            return false;
        }
    }
}