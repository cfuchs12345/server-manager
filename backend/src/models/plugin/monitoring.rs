use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::Script;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeriesType {
    Datetime,
}

impl SeriesType {
    pub fn for_json(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChartyType {
    Bar,
    Line,
}

impl ChartyType {
    pub fn for_json(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MonitioringDef {
    pub pre_process: Option<Script>,
    pub id: String,
    pub name: String,
    pub series_type: SeriesType,
    pub chart_type: ChartyType,
    pub identifier: KeyValue,
    pub sub_identifier: Option<KeyValue>,
    pub values: Vec<KeyValue>,
}

impl MonitioringDef {
    pub fn get_sub_identifiers_as_vec(&self) -> Vec<KeyValue> {
        self.sub_identifier.iter().cloned().collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub name: String,
    pub value_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimeSeriesResponse {
    pub meta_data: TimeSeriesResponseMetaData,
    pub data: TimeSeriesResponseData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimeSeriesResponseMetaData {
    pub ipaddress: String,
    pub series: String,
    pub name: String,
    pub series_id: String,
    pub series_type: String,
    pub chart_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimeSeriesResponseData {
    pub query: String,
    pub columns: Vec<TimeSeriesResponseColumnMetaData>,
    pub dataset: Vec<Vec<Value>>,
    pub count: u32,
}

impl TimeSeriesResponseData {
    pub fn to_response(&self, meta_data: TimeSeriesResponseMetaData) -> TimeSeriesResponse {
        TimeSeriesResponse {
            meta_data,
            data: self.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimeSeriesResponseColumnMetaData {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: String,
}
