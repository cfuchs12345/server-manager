use futures::prelude::*;
use reqwest::Url;
use ssdp_client::SearchTarget;
use std::{
    time::{Duration},
};

use crate::{server_types::{Feature, FeaturesOfServer, Param}, http_functions, plugin_types::Plugin};

const LOCATION: &str = "location";
const UPNP: &str = "upnp";

pub async fn upnp_discover(
    wait_time_for_upnp: usize,
    accept_self_signed_certificates: bool,
    plugins: &[Plugin]
) -> Result<Vec<FeaturesOfServer>, std::io::Error> {
    let mut server_features_with_upnp: Vec<FeaturesOfServer> = Vec::new();

    let found = plugins.iter().find(|p| p.id == UPNP);
    if found.is_none() {
        return Ok(Vec::new());
    }
    let plugin = found.unwrap();

    let search_target = SearchTarget::RootDevice;

    match ssdp_client::search(&search_target, Duration::from_secs( u64::try_from(wait_time_for_upnp).unwrap()), wait_time_for_upnp).await {
        Ok(mut responses) => {
            while let Some(response) = responses.next().await {
                match &response {
                    Ok(res) => {
                        let location = res.location();
                        log::info!("Found UPnP device that responded with location {}", location);

                        match Url::parse(location) {
                            Ok(url) => {
                                server_features_with_upnp.push(FeaturesOfServer {
                                    ipaddress: url.host().unwrap().to_string(),
                                    features: vec![Feature {
                                        id: plugin.id.clone(),
                                        name: plugin.name.clone(),
                                        params: vec![
                                            Param {
                                                name: LOCATION.to_string(),
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

    

    Ok(parse_device_info_from_location(server_features_with_upnp, accept_self_signed_certificates, plugin).await)
}

async fn parse_device_info_from_location(server_features_with_upnp: Vec<FeaturesOfServer>, accept_self_signed_certificates: bool, plugin: &Plugin) ->  Vec<FeaturesOfServer> {
    let clone = server_features_with_upnp.clone();
    for fos in server_features_with_upnp {
        match fos.features.iter().find( |f| f.id == plugin.id) {
            Some(upnp_feature) => {
                match upnp_feature.params.iter().find( |p| p.name == LOCATION) {
                    Some( location_param ) =>  {
                        match http_functions::execute_http_request(location_param.value.clone(), http_functions::GET, None, None, accept_self_signed_certificates).await {
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
            },
            None => {
                log::info!("No UPnP feature found for server {}", fos.ipaddress);
            }           
        }
    }    
    
    clone    
}

fn parse_upnp_description(text: String)  {
        log::info!("response: {}", text);
}
