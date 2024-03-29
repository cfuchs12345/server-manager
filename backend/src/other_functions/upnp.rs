use futures::prelude::*;
use quick_xml::de::from_str;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use ssdp_client::SearchTarget;
use std::{collections::HashMap, net::IpAddr, time::Duration};

use crate::{
    common, datastore,
    models::{
        error::AppError,
        plugin::Plugin,
        server::{Feature, FeaturesOfServer, Param},
    },
};

const LOCATION: &str = "location";
const UPNP: &str = "upnp";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRoot {
    pub spec_version: SpecVersion,
    pub device: Device,
}

impl DeviceRoot {
    pub(crate) fn new() -> DeviceRoot {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpecVersion {
    pub major: u16,
    pub minor: u16,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_type: String,
    pub friendly_name: Option<String>,
    pub manufacturer: Option<String>,
    #[serde(rename = "manufacturerURL")]
    pub manufacturer_url: Option<String>,
    pub model_name: Option<String>,
    pub model_description: Option<String>,
    pub model_number: Option<String>,
    #[serde(rename = "presentationURL")]
    pub presentation_url: Option<String>,

    pub serial_number: Option<String>,
    #[serde(rename = "UDN")]
    pub udn: String,
    pub service_list: ServiceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceList {
    pub service: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub service_type: String,
    pub service_id: String,
    #[serde(rename = "SCPDURL")]
    pub scpd_url: String,
    #[serde(rename = "controlURL")]
    pub control_url: String,
    #[serde(rename = "eventSubURL")]
    pub event_sub_url: String,
}

pub async fn upnp_discover(
    wait_time_for_upnp: usize,
    upnp_activated: bool,
) -> Result<Vec<FeaturesOfServer>, AppError> {
    if !upnp_activated {
        log::debug!("Skipping UPnP device discovery since the plugin is disabled");
        return Ok(Vec::new());
    }

    let mut serverfeature_by_location: HashMap<String, FeaturesOfServer> = HashMap::new();

    let Some(plugin) = datastore::get_plugin(UPNP)? else {
        log::debug!("UPNP Plugin not found");
        return Ok(Vec::new());
    };

    let search_target = SearchTarget::RootDevice;
    let secs =
        u64::try_from(wait_time_for_upnp).map_err(|err| AppError::Unknown(format!("{}", err)))?;

    match ssdp_client::search(
        &search_target,
        Duration::from_secs(secs),
        wait_time_for_upnp,
    )
    .await
    {
        Ok(mut responses) => {
            while let Some(response) = responses.next().await {
                match &response {
                    Ok(res) => {
                        let location = res.location();
                        if !serverfeature_by_location.contains_key(location) {
                            log::debug!(
                                "Found UPnP device that responded with location {}",
                                location
                            );
                        } else {
                            continue;
                        }

                        match Url::parse(location) {
                            Ok(url) => {
                                let host_address: IpAddr = url
                                    .host()
                                    .ok_or(AppError::Unknown(format!(
                                        "Url {} seems to be invalid",
                                        url
                                    )))?
                                    .to_string()
                                    .parse()?;

                                serverfeature_by_location.insert(
                                    location.to_string(),
                                    FeaturesOfServer {
                                        ipaddress: host_address,
                                        features: vec![Feature {
                                            id: plugin.id.clone(),
                                            name: plugin.name.clone(),
                                            params: vec![Param {
                                                name: LOCATION.to_string(),
                                                value: location.to_string(),
                                            }],
                                            ..Default::default()
                                        }],
                                    },
                                );
                            }
                            Err(err) => {
                                log::error!("Error while parsing url {} {}", location, err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error while extracting response {:?} {}", response, err);
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Error while reading responses: {}", err);
        }
    }
    log::debug!(
        "UPnP device discovery done. Found {} distinct devices",
        serverfeature_by_location.len()
    );
    Ok(serverfeature_by_location
        .iter()
        .map(|e| e.1.to_owned())
        .collect())
}

#[allow(dead_code)]
pub async fn parse_device_info_from_location(
    server_features_with_upnp: Vec<FeaturesOfServer>,
    plugin: &Plugin,
) -> Result<Vec<FeaturesOfServer>, AppError> {
    let clone = server_features_with_upnp.clone();
    for fos in server_features_with_upnp {
        match fos.features.iter().find(|f| f.id == plugin.id) {
            Some(upnp_feature) => {
                log::debug!("server {} uses the plugin {}", fos.ipaddress, plugin.id);

                match upnp_feature.params.iter().find(|p| p.name == LOCATION) {
                    Some(location_param) => {
                        log::debug!(
                            "found location {} for UPnP device {}",
                            location_param.value,
                            fos.ipaddress
                        );

                        match common::execute_http_request(
                            location_param.value.as_str(),
                            common::GET,
                            None,
                            None,
                        )
                        .await
                        {
                            Ok(text) => {
                                log::debug!(
                                    "executed request on location {} of UPnP device {}",
                                    location_param.value,
                                    fos.ipaddress
                                );

                                parse_upnp_description(text.as_str())?;
                            }
                            Err(err) => {
                                log::error!("Error while doing http request on location {} of UPnP device {}. Error {}", location_param.value.clone(), fos.ipaddress, err);
                            }
                        }
                    }
                    None => {
                        log::error!("No location found for UPnP feature of server {}. Even if it was found as a UPnP device.", fos.ipaddress);
                    }
                }
            }
            None => {
                log::warn!("No UPnP feature found for server {}", fos.ipaddress);
            }
        }
    }

    Ok(clone)
}

pub fn parse_upnp_description(text: &str) -> Result<DeviceRoot, AppError> {
    from_str::<DeviceRoot>(text)
        .map_err(|err| AppError::Unknown(format!("Could not parse XML. Error was: {:?}", err)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_upnp_description() {
        let text = r#"<?xml version="1.0"?>
        <root xmlns="urn:schemas-upnp-org:device-1-0">
        <specVersion>
        <major>1</major>
        <minor>0</minor>
        </specVersion>
        <device>
        <deviceType>urn:schemas-wifialliance-org:device:WFADevice:1</deviceType>
        <friendlyName>WPS Access Point</friendlyName>
        <manufacturer>ASUSTeK Computer Inc.</manufacturer>
        <modelName>Wi-Fi Protected Setup Router</modelName>
        <modelNumber>ZenWiFi_XT8</modelNumber>
        <serialNumber>00:00:00:00:00:00</serialNumber>
        <UDN>uuid:0dfcaeec-c25a-9132-0000-0000000000</UDN>
        <serviceList>
        <service>
        <serviceType>urn:schemas-wifialliance-org:service:WFAWLANConfig:1</serviceType>
        <serviceId>urn:wifialliance-org:serviceId:WFAWLANConfig1</serviceId>
        <SCPDURL>wps_scpd.xml</SCPDURL>
        <controlURL>wps_control</controlURL>
        <eventSubURL>wps_event</eventSubURL>
        </service>
        </serviceList>
        </device>
        </root>"#;
        let parsed = parse_upnp_description(text);
        assert!(parsed.is_ok());
        let unwrapped = parsed.expect("should not happen");

        assert_eq!(
            unwrapped.device.manufacturer.expect("should not happen"),
            "ASUSTeK Computer Inc."
        );
        assert_eq!(unwrapped.device.service_list.service.len(), 1);
        assert_eq!(
            unwrapped
                .device
                .service_list
                .service
                .first()
                .expect("should not happen")
                .control_url,
            "wps_control"
        );
    }
}
