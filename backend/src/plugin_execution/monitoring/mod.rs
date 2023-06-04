use jsonpath_rust::JsonPathQuery;
use std::{
    collections::HashMap,
    net::IpAddr,
    time::{Duration, SystemTime},
};

use crate::{
    datastore::{self, TimeSeriesData, TimeSeriesPersistence, Value},
    models::{
        error::AppError,
        plugin::{
            data::{KeyValue, Monitioring, SeriesType},
            Plugin,
        },
        server::Server,
    },
};

pub async fn get_monitoring_data(series_id: &str, ipaddress: IpAddr) -> Result<String, AppError> {
    let monitoring = get_monitoring_config_for_series(series_id).ok_or(AppError::Unknown(
        format!("Could not find monitoring config for series {}", series_id),
    ))?;

    log::trace!("querying monitoring data for {}", series_id);

    let config = datastore::QuestDBConfig::new();
    let host = config.get_host();
    let port = config.get_http_port();

    let select = create_data_select(&monitoring, series_id, format!("{}", ipaddress).as_str());

    let query = vec![("nm", "true"), ("query", select.as_str())];

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let url = format!("http://{}:{}/exec", host, port);

    let result = client.get(url).query(&query).send().await?;

    let text = result.text().await?;

    log::info!("response from db: {}", text);

    let value = serde_json::from_str::<serde_json::Value>(&text)?;

    let enriched = enrich_response(value, ipaddress, series_id, &monitoring)?;

    serde_json::to_string(&enriched).map_err(AppError::from)
}

fn create_data_select(monitoring: &Monitioring, series_id: &str, identifier: &str) -> String {
    let mut cols: Vec<String> = Vec::new();
    cols.push(monitoring.identifier.name.clone());
    if let Some(sub_identifier) = &monitoring.sub_identifier {
        cols.push(sub_identifier.name.clone());
    }
    cols.push(format!("last({})", monitoring.value.name.clone()));
    cols.push("timestamp".to_owned()); // always the last column

    let where_stmnt = "timestamp > sysdate() - 36000000000L";
    let sample_by = "SAMPLE BY 1m FILL(0)";

    format!(
        "select {} from {} where {} AND {} = '{}' {} ORDER BY timestamp asc",
        cols.join(","),
        series_id,
        where_stmnt,
        monitoring.identifier.name,
        identifier,
        sample_by
    )
}

fn get_monitoring_config_for_series(series_id: &str) -> Option<Monitioring> {
    if series_id == "server_status" {
        Some(Monitioring {
            pre_process: None,
            id: "server_status".to_owned(),
            name: "Server Status".to_owned(),
            chart_type: crate::models::plugin::data::ChartyType::line,
            series_type: SeriesType::datetime,
            identifier: KeyValue {
                name: "IP".to_owned(),
                value_type: "symbol".to_owned(),
                value: "".to_owned(),
            },
            sub_identifier: None,
            value: KeyValue {
                name: "running".to_owned(),
                value_type: "integer".to_owned(),
                value: "".to_owned(),
            },
        })
    } else {
        datastore::get_monitoring_config_for_series(series_id)
    }
}

fn enrich_response(
    json: serde_json::Value,
    ipaddress: IpAddr,
    series: &str,
    monitoring: &Monitioring,
) -> Result<serde_json::Value, AppError> {
    let value: serde_json::Value = match json {
        serde_json::Value::Object(mut map) => {
            map.insert(
                "ipaddress".to_owned(),
                serde_json::Value::String(format!("{}", ipaddress)),
            );
            map.insert(
                "series".to_owned(),
                serde_json::Value::String(series.to_owned()),
            );
            map.insert(
                "name".to_owned(),
                serde_json::Value::String(monitoring.name.clone()),
            );
            map.insert(
                "series_id".to_owned(),
                serde_json::Value::String(monitoring.id.clone()),
            );
            map.insert(
                "series_type".to_owned(),
                serde_json::Value::String(format!("{:?}", monitoring.series_type)),
            );
            map.insert(
                "chart_type".to_owned(),
                serde_json::Value::String(format!("{:?}", monitoring.chart_type)),
            );
            map.insert(
                "identifier".to_owned(),
                serde_json::Value::String(monitoring.identifier.name.clone()),
            );
            if let Some(sub_identifier) = &monitoring.sub_identifier {
                map.insert(
                    "sub_identifier".to_owned(),
                    serde_json::Value::String(sub_identifier.name.clone()),
                );
            }
            map.insert(
                "value".to_owned(),
                serde_json::Value::String(monitoring.value.name.clone()),
            );
            serde_json::Value::Object(map)
        }
        y => y,
    };
    Ok(value)
}

pub async fn monitor_all() -> Result<(), AppError> {
    let servers = datastore::get_all_servers();
    let plugins = datastore::get_all_plugins();
    let crypto_key = datastore::get_crypto_key();

    let relevant_plugins: Vec<Plugin> = plugins
        .iter()
        .filter(|p| p.data.iter().any(|d| !d.monitoring.is_empty()))
        .map(|p| p.to_owned())
        .collect();

    let mut map: HashMap<String, Vec<TimeSeriesData>> = HashMap::new();

    log::debug!("relevant plugins for monitoring: {:?}", &relevant_plugins);

    for server in servers {
        for plugin in &relevant_plugins {
            let feature = server.find_feature(plugin.id.as_str());

            if let Some(feature) = feature {
                log::trace!("Server {:?} has relevant feature {:?}", server, feature);

                for data in &plugin.data {
                    let result = super::data::execute_specific_data_query(
                        &server,
                        plugin,
                        &feature,
                        data,
                        None,
                        crypto_key.as_str(),
                    )
                    .await;

                    log::trace!("result from monitoring is {:?}", result);

                    if let Ok(Some(str)) = result {
                        for monitoring in &data.monitoring {
                            let mut extracted_data =
                                extract_monitoring_data(&str, &server, monitoring)?;

                            map.entry(monitoring.id.clone())
                                .or_insert_with(Vec::new)
                                .append(&mut extracted_data);
                        }
                    }
                }
            } else {
                log::debug!(
                    "Feature {:?} not relevant for Server {:?} it only has following features: {:?}",
                    feature,
                    server,
                    server.features
                );
            }
        }
    }

    let mut persistence = TimeSeriesPersistence::new().await;

    for (series_id, data_vec) in map {
        log::trace!("Saving time series data for {}: {:?}", series_id, data_vec);
        datastore::save_timeseries_data(&mut persistence, series_id.as_str(), data_vec.clone())
            .await?;
    }

    Ok(())
}

fn extract_monitoring_data(
    response: &str,
    server: &Server,
    monitoring: &Monitioring,
) -> Result<Vec<TimeSeriesData>, AppError> {
    let mut vec = Vec::new();

    let response = match &monitoring.pre_process {
        Some(script) => super::pre_or_post_process(response, script)?,
        None => response.to_owned(),
    };

    match serde_json::from_str::<serde_json::Value>(response.as_str()) {
        Ok(value) => {
            let identifiers = get_values(Some(monitoring.identifier.clone()), &value, server);
            let sub_identifiers = get_values(monitoring.sub_identifier.clone(), &value, server);
            let values = get_values(Some(monitoring.value.clone()), &value, server);

            for i in 0..values.len() {
                let identifiers_for_index = get_val_at_index_or_single_value(i, &identifiers);
                let sub_identifiers_for_index =
                    get_val_at_index_or_single_value(i, &sub_identifiers);
                let values_for_index = get_val_at_index_or_single_value(i, &values);

                if identifiers_for_index.is_none() || values_for_index.is_none() {
                    continue;
                }

                vec.push(TimeSeriesData {
                    identifier: identifiers_for_index.unwrap(),
                    sub_identifier: sub_identifiers_for_index,
                    value: values_for_index.unwrap(),
                    timestamp: datastore::Timestamp::SysTime(SystemTime::now()),
                });
            }
        }
        Err(err) => {
            return Err(AppError::ParseError(format!(
                "Could not parse response as JSON. Error: {}",
                err
            )))
        }
    }

    Ok(vec)
}

fn get_values(field: Option<KeyValue>, value: &serde_json::Value, server: &Server) -> Vec<Value> {
    let mut vec = Vec::new();

    if field.is_none() {
        return vec;
    }

    let field = field.unwrap();

    match field.value.as_str() {
        "${IP}" => {
            if let Some(value) = make_value(&field, format!("{}", server.ipaddress).as_str()) {
                vec.push(value);
            }
        }
        y => {
            if y.starts_with('$') {
                match value.clone().path(y) {
                    Ok(res) => {
                        log::debug!("Json path query result for {} is {}", y, res);

                        let list = match res.is_array() {
                            true => res
                                .as_array()
                                .unwrap()
                                .iter()
                                .flat_map(convert_value_to_str)
                                .collect(),
                            false => vec![convert_value_to_str(&res).unwrap_or_default()],
                        };

                        log::debug!("Processing values {:?} for {}", list, field.name);

                        let mut key_values: Vec<Value> = list
                            .iter()
                            .flat_map(|str| make_value(&field, str))
                            .collect();

                        vec.append(&mut key_values);
                    }
                    Err(err) => {
                        log::error!("error during json path query: {}", err);
                    }
                }
            } else if let Some(value) = make_value(&field, y) {
                log::warn!(
                    "value {} doesn't start with $. Trating it as a constant value",
                    y
                );

                vec.push(value);
            }
        }
    };

    vec
}

fn convert_value_to_str(value: &serde_json::Value) -> Option<String> {
    if value.is_f64() {
        value.as_f64().map(|n| n.to_string())
    } else if value.is_i64() {
        value.as_i64().map(|n| n.to_string())
    } else if value.is_u64() {
        value.as_u64().map(|n| n.to_string())
    } else if value.is_boolean() {
        value.as_bool().map(|b| b.to_string())
    } else if value.is_string() {
        value.as_str().map(|s| s.to_owned())
    } else {
        log::warn!("Unhandled json type for value {:?}", value);
        None
    }
}

fn make_value(field: &KeyValue, value: &str) -> Option<Value> {
    match field.value_type.as_str() {
        "symbol" => Some(Value::Symbol(field.name.to_owned(), value.to_owned())),
        "boolean" => match value {
            "1" => Some(Value::Bool(field.name.to_owned(), true)), // 1 is also traeted as true
            "0" => Some(Value::Bool(field.name.to_owned(), false)), // 0 is traeted as false
            y => Some(Value::Bool(
                field.name.to_owned(),
                y.to_lowercase().parse().unwrap_or_default(),
            )),
        },
        "integer" => Some(Value::Int(
            field.name.to_owned(),
            value.parse().unwrap_or_default(),
        )),
        "float" => Some(Value::Float(
            field.name.to_owned(),
            value.parse().unwrap_or_default(),
        )),
        "string" => Some(Value::String(field.name.to_owned(), value.to_owned())),
        y => {
            log::error!("unknown field type {} found", y);
            None
        }
    }
}

fn get_val_at_index_or_single_value(index: usize, values: &[Value]) -> Option<Value> {
    let val_at_index = values.get(index);

    if val_at_index.is_some() {
        val_at_index.map(|v| v.to_owned())
    } else if values.len() == 1 {
        values.get(0).map(|v| v.to_owned())
    } else {
        None
    }
}
