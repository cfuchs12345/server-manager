use std::fmt::Debug;

use questdb::ingress::{Buffer, Sender, SenderBuilder};

use crate::models::{
    error::AppError,
    timeseries::{TimeSeriesData, TimeSeriesValue},
};

#[derive(Clone, Debug)]
pub struct QuestDBConfig {
    host: String,
    influx_port: u16,
    http_port: u16,
}

impl QuestDBConfig {
    pub fn get_host(&self) -> String {
        self.host.clone()
    }
    pub fn get_http_port(&self) -> u16 {
        self.http_port
    }
}

impl QuestDBConfig {
    fn new() -> Result<Self, AppError> {
        let config = super::get_config()?;

        let timeseries_db_host = config.get_string("timeseries_db_host")?;

        let timeseries_influx_port = config.get_int("timeseries_db_influx_port")?;

        let timeseries_http_port = config.get_int("timeseries_db_http_port")?;

        Ok(QuestDBConfig {
            host: timeseries_db_host,
            influx_port: timeseries_influx_port as u16,
            http_port: timeseries_http_port as u16,
        })
    }
}

pub fn get_timeseriesdb_config() -> Result<QuestDBConfig, AppError> {
    QuestDBConfig::new()
}

#[derive(Debug)]
pub struct TimeSeriesPersistence {
    host: String,
    port: u16,
    sender: Option<Sender>,
}

impl Clone for TimeSeriesPersistence {
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
            sender: None,
        }
    }
}

pub async fn save_timeseries_data(
    timeseries_persistence: &mut TimeSeriesPersistence,
    series_id: &str,
    data_vec: Vec<TimeSeriesData>,
) -> Result<(), AppError> {
    timeseries_persistence.save(series_id, data_vec).await
}

impl TimeSeriesPersistence {
    pub async fn new() -> Result<TimeSeriesPersistence, AppError> {
        let config = QuestDBConfig::new()?;

        Ok(TimeSeriesPersistence {
            host: config.host,
            port: config.influx_port,
            sender: None,
        })
    }

    pub async fn save(
        &mut self,
        series_id: &str,
        data_vec: Vec<TimeSeriesData>,
    ) -> Result<(), AppError> {
        if data_vec.is_empty() {
            return Ok(());
        }

        if self.sender.is_none() {
            self.create_sender()?;
        }

        if let Some(sender) = self.sender.as_mut() {
            let vec = data_vec;

            for data in vec {
                let mut buffer = Buffer::new();
                buffer
                    .table(series_id)
                    .map_err(|err| AppError::DatabaseError(format!("{}", err)))?;

                let mut all_columns: Vec<TimeSeriesValue> = Vec::new();
                all_columns.push(data.identifier);

                for sub_id in data.sub_identifiers {
                    all_columns.push(sub_id);
                }

                all_columns.push(data.value);

                // first add all symbols
                for value in &all_columns {
                    if !value.is_symbol() {
                        continue;
                    }
                    value.apply(&mut buffer)?;
                }

                for value in &all_columns {
                    if value.is_symbol() {
                        continue;
                    }
                    value.apply(&mut buffer)?;
                }

                data.timestamp.apply(&mut buffer)?;

                sender
                    .flush(&mut buffer)
                    .map_err(|err| AppError::DatabaseError(format!("{}", err)))?;
            }
        }
        Ok(())
    }

    fn create_sender(&mut self) -> Result<(), AppError> {
        self.sender = Some(
            SenderBuilder::new(self.host.clone(), self.port)
                .connect()
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
        );
        Ok(())
    }
}
