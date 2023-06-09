use std::{collections::HashMap, net::IpAddr, time::Duration};

use crate::{
    common,
    datastore::{self, TimeSeriesData, TimeSeriesPersistence},
    models::{
        error::AppError,
        plugin::{
            monitoring::{
                ChartyType, KeyValue, Monitioring, SeriesType, TimeSeriesResponse,
                TimeSeriesResponseData, TimeSeriesResponseMetaData,
            },
            Plugin,
        },
    },
};

mod response_parser;

pub async fn get_monitoring_data(
    series_id: &str,
    ipaddress: IpAddr,
) -> Result<TimeSeriesResponse, AppError> {
    let monitoring = get_monitoring_config_for_series(series_id)?.ok_or(AppError::Unknown(
        format!("Could not find monitoring config for series {}", series_id),
    ))?;

    log::trace!("querying monitoring data for {}", series_id);

    let config = datastore::QuestDBConfig::new()?;

    let select = create_data_select(&monitoring, series_id, format!("{}", ipaddress).as_str());

    let query = vec![("query", select.as_str())];

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()?;

    let result = client
        .get(format!(
            "http://{}:{}/exec",
            config.get_host(),
            config.get_http_port()
        ))
        .query(&query)
        .send()
        .await?
        .text()
        .await?;

    log::trace!("response from db: {}", result);

    let value = serde_json::from_str::<TimeSeriesResponseData>(&result)?;

    enrich_response(value, ipaddress, series_id, &monitoring)
}

fn create_data_select(monitoring: &Monitioring, series_id: &str, identifier: &str) -> String {
    let mut cols: Vec<String> = Vec::new();
    cols.push(common::IDENTIFIER.to_owned());

    if monitoring.sub_identifier.is_some() {
        cols.push(common::SUB_IDENTIFIER.to_owned());
    }

    if monitoring.values.len() > 1 {
        cols.push(common::SUB_IDENTIFIER_2.to_owned());
    }

    cols.push(format!("last({})", common::VALUE.to_owned()));

    cols.push(common::TIMESTAMP.to_owned()); // always the last column

    let where_stmnt = format!("{} > sysdate() - 36000000000L", common::TIMESTAMP);
    let sample_by = "SAMPLE BY 1m FILL(NONE)";

    format!(
        "select {} from {} where {} AND {} = '{}' {} ORDER BY {} asc",
        cols.join(","),
        series_id,
        where_stmnt,
        common::IDENTIFIER,
        identifier,
        sample_by,
        common::TIMESTAMP
    )
}

fn get_monitoring_config_for_series(series_id: &str) -> Result<Option<Monitioring>, AppError> {
    if series_id == "server_status" {
        Ok(Some(Monitioring {
            pre_process: None,
            id: "server_status".to_owned(),
            name: "Server Status".to_owned(),
            chart_type: ChartyType::Line,
            series_type: SeriesType::Datetime,
            identifier: KeyValue {
                name: "IP".to_owned(),
                value_type: "symbol".to_owned(),
                value: "".to_owned(),
            },
            sub_identifier: None,
            values: vec![KeyValue {
                name: "running".to_owned(),
                value_type: "integer".to_owned(),
                value: "".to_owned(),
            }],
        }))
    } else {
        datastore::get_monitoring_config_for_series(series_id)
    }
}

fn enrich_response(
    data: TimeSeriesResponseData,
    ipaddress: IpAddr,
    series: &str,
    monitoring: &Monitioring,
) -> Result<TimeSeriesResponse, AppError> {
    let meta_data = TimeSeriesResponseMetaData {
        ipaddress: format!("{}", ipaddress),
        series: series.to_owned(),
        series_id: monitoring.id.clone(),
        name: monitoring.name.to_owned(),
        series_type: monitoring.series_type.for_json(),
        chart_type: monitoring.chart_type.for_json(),
    };

    Ok(data.to_response(meta_data))
}

pub async fn monitor_all(silent: &bool) -> Result<(), AppError> {
    let servers = datastore::get_all_servers()?;
    let plugins = datastore::get_all_plugins()?;
    let crypto_key = datastore::get_crypto_key()?;

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
                    if data.monitoring.is_empty() {
                        continue;
                    }

                    let input_response_tuples_result = super::data::execute_specific_data_query(
                        &server,
                        plugin,
                        &feature,
                        data,
                        None,
                        crypto_key.as_str(),
                        silent, // silent - no error log
                    )
                    .await;

                    if let Ok(input_response_tuples) = input_response_tuples_result {
                        let parser = response_parser::MonitoringDataExtractor::new(
                            input_response_tuples,
                            data,
                        );

                        let parsed_data = parser.parse()?;

                        map.extend(parsed_data);
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

    let mut persistence = TimeSeriesPersistence::new().await?;

    for (series_id, data_vec) in map {
        log::trace!("Saving time series data for {}: {:?}", series_id, data_vec);
        datastore::save_timeseries_data(&mut persistence, series_id.as_str(), data_vec.clone())
            .await?;
    }

    Ok(())
}
