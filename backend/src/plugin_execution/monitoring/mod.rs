use std::{collections::HashMap, net::IpAddr};

use std::time::{Duration, Instant};

use crate::{
    commands::CommandInput,
    common,
    datastore::{self, TimeSeriesData, TimeSeriesPersistence},
    models::{
        error::AppError,
        plugin::{
            data::DataDef,
            monitoring::{
                ChartyType, KeyValue, MonitioringDef, SeriesType, TimeSeriesResponse,
                TimeSeriesResponseData, TimeSeriesResponseMetaData,
            },
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

fn create_data_select(monitoring: &MonitioringDef, series_id: &str, identifier: &str) -> String {
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

fn get_monitoring_config_for_series(series_id: &str) -> Result<Option<MonitioringDef>, AppError> {
    if series_id == "server_status" {
        Ok(Some(MonitioringDef {
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
    monitoring: &MonitioringDef,
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

pub struct MonitoringProcessor {
    map: HashMap<String, Vec<TimeSeriesData>>,
    time_reached: bool,
}

impl MonitoringProcessor {
    pub fn new(last_run: Option<Instant>, interval: Duration) -> Self {
        MonitoringProcessor {
            map: HashMap::new(),
            time_reached: last_run.is_none()
                || last_run
                    .unwrap()
                    .checked_add(interval)
                    .unwrap()
                    .lt(&Instant::now()),
        }
    }

    pub fn is_relevant_data_for_processing(&self, data: &DataDef) -> bool {
        !data.monitoring.is_empty() || !self.time_reached
    }

    pub async fn process(
        &mut self,
        data: &DataDef,
        input_response_tuples: &[(CommandInput, String)],
    ) -> Result<(), AppError> {
        if !self.is_relevant_data_for_processing(data) {
            return Ok(());
        }

        let parser = response_parser::MonitoringDataExtractor::new(input_response_tuples, data);

        let parsed_data = parser.parse()?;

        self.map.extend(parsed_data);

        Ok(())
    }

    pub async fn finish(self) -> Result<(), AppError> {
        let mut persistence = TimeSeriesPersistence::new().await?;

        for (series_id, data_vec) in self.map {
            log::trace!("Saving time series data for {}: {:?}", series_id, data_vec);
            datastore::save_timeseries_data(&mut persistence, series_id.as_str(), data_vec.clone())
                .await?;
        }
        Ok(())
    }
}
