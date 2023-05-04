use futures::prelude::*;
use reqwest::Url;
use ssdp_client::SearchTarget;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    time::{Duration, Instant},
};

use crate::server_types::{Feature, FeaturesOfServer};

pub async fn upnp_discover(
    wait_time_for_upnp: usize,
) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let mut feature_of_server_list: Vec<FeaturesOfServer> = Vec::new();

    let search_target = SearchTarget::RootDevice;

    match ssdp_client::search(&search_target, Duration::from_secs(10), 5).await {
        Ok(mut responses) => {
            while let Some(response) = responses.next().await {
                match &response {
                    Ok(res) => {
                        let location = res.location();

                        match Url::parse(location) {
                            Ok(url) => {
                                feature_of_server_list.push(FeaturesOfServer {
                                    ipaddress: url.host().unwrap().to_string(),
                                    features: vec![Feature {
                                        id: "upnp".to_string(),
                                        name: "upnp".to_string(),                                        
                                        ..Default::default()
                                    }],
                                });
                            },
                            Err(err) => {
                                log::error!("Error while parsing url {} {}", location, err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error while extracting response {:?} {}", response, err);
                    }
                }
                //feature_of_server_list.push( FeaturesOfServer { ipaddress: response., features: () })
            }
        }
        Err(err) => {
            log::error!("Error while reading responses: {}", err);
        }
    }

    Ok(feature_of_server_list)
}
