use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    time::{Duration, Instant},
};
use tokio::net::UdpSocket;

use crate::server_types::{Feature, FeaturesOfServer};

pub async fn upnp_discover(duration: Duration) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let mut feature_of_server_list: Vec<FeaturesOfServer> = Vec::new();

    let any: SocketAddr = ([0, 0, 0, 0], 0).into();
    let socket = UdpSocket::bind(any).await?;
    match socket.join_multicast_v4(Ipv4Addr::new(239, 255, 255, 250), Ipv4Addr::new(0, 0, 0, 0)) {
        Ok(_res) => {
            // Set the socket address to the multicast IP and port for UPnP device discovery
            let socket_addr: SocketAddr = ([239, 255, 255, 250], 1900).into();

            let request_msg = get_request_message(socket_addr);

            let start = Instant::now();
            let wait_time_for_upnp: u64 = 15; // seconds

            // Send the discovery request
            match socket.send_to(request_msg.as_bytes(), &socket_addr).await {
                Ok(_result) => {
                    loop {
                        if time_is_over(start, wait_time_for_upnp) {
                            log::info!("Time is over. Stopping UPnP device discovery");
                            break;
                        }

                        async fn get_next(socket: &UdpSocket) -> Result<String, std::io::Error> {
                            // Receive the discovery response
                            let mut buf = [0; 2048];
                            let (size, _) = socket.recv_from(&mut buf).await?;
                            // Convert the response to a string
                            let response = std::str::from_utf8(unsafe {
                                std::slice::from_raw_parts(buf.as_ptr() as *const u8, size)
                            })
                            .map_err(|err| {
                                std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Could not read response.",
                                )
                            })?;

                            log::info!("reponse was: {}", response);

                            let headers = parse_raw_http_response(response)?;
                            let location = headers.get("location").ok_or_else(|| {
                                std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Response header missing location",
                                )
                            })?;
                            Ok(location.to_string())
                        }

                        match get_next(&socket).await {
                            Ok(location) => feature_of_server_list.push(FeaturesOfServer {
                                ipaddress: location,
                                features: vec![Feature {
                                    name: "upnp".to_owned(),
                                    ..Default::default()
                                }],
                            }),
                            Err(err) => {
                                log::error!("Error while reading from socket: {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error while sending from socket: {}", err);
                }
            }
        }
        Err(err) => {
            log::error!("Error while trying to joind multicast group: {}", err);
        }
    }

    Ok(feature_of_server_list)
}

fn time_is_over(start: Instant, seconds: u64) -> bool { 
    start.elapsed() > Duration::from_secs(seconds)
}

fn get_request_message(socket_addr: SocketAddr) -> String {
    let message = format!(
        "M-SEARCH * HTTP/1.1\nHOST: {}:{}\nMAN: \"ssdp:discover\"\nMX: 2\r\nST: ssdp:all\n\n",
        socket_addr.ip(),
        socket_addr.port()
    );
    log::info!(
        "Using request message {} for UPnP device discovery.",
        message
    );

    message
}

fn parse_raw_http_response(response_str: &str) -> Result<HashMap<String, &str>, std::io::Error> {
    let mut headers = HashMap::new();

    match response_str.split("\r\n\r\n").next() {
        Some(header_str) => {
            for header_line in header_str.split("\r\n") {
                if let Some(colon_index) = header_line.find(':') {
                    let header_name = header_line[0..colon_index].to_ascii_lowercase();
                    let header_value = header_line[colon_index + 1..].trim();
                    headers.insert(header_name, header_value);
                }
            }
            Ok(headers)
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid HTTP response",
        )),
    }
}
