use futures::prelude::*;
use reqwest::Url;
use ssdp_client::SearchTarget;
use std::{
    time::{Duration},
};

use crate::{server_types::{Feature, FeaturesOfServer, Param}, http_functions};

pub async fn upnp_discover(
    wait_time_for_upnp: usize,
    accept_self_signed_certificates: bool
) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let mut server_features_with_upnp: Vec<FeaturesOfServer> = Vec::new();

    let search_target = SearchTarget::RootDevice;

    match ssdp_client::search(&search_target, Duration::from_secs( u64::try_from(wait_time_for_upnp).unwrap()), wait_time_for_upnp).await {
        Ok(mut responses) => {
            while let Some(response) = responses.next().await {
                match &response {
                    Ok(res) => {
                        let location = res.location();

                        match Url::parse(location) {
                            Ok(url) => {
                                server_features_with_upnp.push(FeaturesOfServer {
                                    ipaddress: url.host().unwrap().to_string(),
                                    features: vec![Feature {
                                        id: "upnp".to_string(),
                                        name: "upnp".to_string(),
                                        params: vec![
                                            Param {
                                                name: "Location".to_string(),
                                                value: location.to_string()
                                            }
                                        ],                                
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

    

    Ok(parse_device_info_from_location(server_features_with_upnp, accept_self_signed_certificates).await)
}

async fn parse_device_info_from_location(server_features_with_upnp: Vec<FeaturesOfServer>, accept_self_signed_certificates: bool) ->  Vec<FeaturesOfServer> {
    let clone = server_features_with_upnp.clone();
    for fos in server_features_with_upnp {
        for f in  fos.features {
            match f.params.iter().find( |f| f.name == "location") {
                Some(location_param) => {
                    match http_functions::execute_http_request(location_param.value.clone(), "get", None, None, accept_self_signed_certificates).await {
                        Ok(res) => {
                            match res.text().await {
                                Ok(text) => {
                                    parse_upnp_description(text);
                                },
                                Err(err) => {
                                    log::error!("Error while reading response text from location {} of UPnP device {}. Error {}", location_param.value.clone(), fos.ipaddress, err);
                                }
                            }
                        },
                        Err(err) => {
                            log::error!("Error while doing http request on location {} of UPnP device {}. Error {}", location_param.value.clone(), fos.ipaddress, err);
                        }
                    }
                },
                None => {
                    log::info!("No location found for UPnP feature of server {}", fos.ipaddress);
                }
            }
        }
    }
    
    
    clone    
}

fn parse_upnp_description(text: String)  {
        log::info!("response: {}", text);
}
