use questdb::ingress::Buffer;

use super::error::AppError;

#[derive(Clone, Debug)]
pub struct TimeSeriesData {
    pub identifier: TimeSeriesValue,
    pub sub_identifiers: Vec<TimeSeriesValue>,
    pub value: TimeSeriesValue,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug)]
pub enum TimeSeriesValue {
    Bool(String, bool),
    Int(String, i64),
    Float(String, f64),
    String(String, String),
    Symbol(String, String),
}

impl TimeSeriesValue {
    pub fn apply(&self, buffer: &mut Buffer) -> Result<(), AppError> {
        match self {
            TimeSeriesValue::Bool(name, value) => buffer
                .column_bool(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            TimeSeriesValue::Float(name, value) => buffer
                .column_f64(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            TimeSeriesValue::Int(name, value) => buffer
                .column_i64(name.as_str(), value.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            TimeSeriesValue::String(name, value) => buffer
                .column_str(name.as_str(), value)
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
            TimeSeriesValue::Symbol(name, value) => buffer
                .symbol(name.as_str(), value.as_str())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
        };
        Ok(())
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, TimeSeriesValue::Symbol(_, _))
    }
}

#[derive(Clone, Debug)]
pub enum Timestamp {
    SysTime(std::time::SystemTime),
}

impl Timestamp {
    pub fn apply(&self, buffer: &mut Buffer) -> Result<(), AppError> {
        match self {
            Timestamp::SysTime(timestamp) => buffer
                .at(timestamp.to_owned())
                .map_err(|err| AppError::DatabaseError(format!("{}", err)))?,
        };
        Ok(())
    }
}
