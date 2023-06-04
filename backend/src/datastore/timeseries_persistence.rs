use std::fmt::Debug;

use questdb::ingress::{Buffer, Sender, SenderBuilder};

use crate::models::error::AppError;

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
    pub fn new() -> Self {
        let timeseries_db_host = super::get_config()
            .get_string("timeseries_db_host")
            .unwrap();

        let timeseries_influx_port = super::get_config()
            .get_int("timeseries_db_influx_port")
            .unwrap();

        let timeseries_http_port = super::get_config()
            .get_int("timeseries_db_http_port")
            .unwrap();

        QuestDBConfig {
            host: timeseries_db_host,
            influx_port: timeseries_influx_port as u16,
            http_port: timeseries_http_port as u16,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeSeriesData {
    pub identifier: Value,
    pub sub_identifier: Option<Value>,
    pub value: Value,
    pub timestamp: Timestamp,
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

#[derive(Clone, Debug)]
pub enum Value {
    Bool(String, bool),
    Int(String, i64),
    Float(String, f64),
    String(String, String),
    Symbol(String, String),
}

impl Value {
    fn apply(&self, buffer: &mut Buffer) -> Result<(), AppError> {
        match self {
            Value::Bool(name, value) => buffer
                .column_bool(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            Value::Float(name, value) => buffer
                .column_f64(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            Value::Int(name, value) => buffer
                .column_i64(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            Value::String(name, value) => buffer
                .column_str(name.as_str(), value)
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            Value::Symbol(name, value) => buffer
                .symbol(name.as_str(), value.as_str())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
        };
        Ok(())
    }

    fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_, _))
    }
}

#[derive(Clone, Debug)]
pub enum Timestamp {
    SysTime(std::time::SystemTime),
}

impl Timestamp {
    fn apply(&self, buffer: &mut Buffer) -> Result<(), AppError> {
        match self {
            Timestamp::SysTime(timestamp) => buffer
                .at(timestamp.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
        };
        Ok(())
    }
}

impl TimeSeriesPersistence {
    pub async fn new() -> TimeSeriesPersistence {
        let config = QuestDBConfig::new();

        TimeSeriesPersistence {
            host: config.host,
            port: config.influx_port,
            sender: None,
        }
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

                let mut all_columns: Vec<Value> = Vec::new();
                all_columns.push(data.identifier);
                if let Some(subid) = data.sub_identifier {
                    all_columns.push(subid);
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
