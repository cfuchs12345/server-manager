use jsonpath_rust::JsonPathQuery;

use std::{collections::HashMap, time::SystemTime};

use serde_json::Value;

use crate::{
    commands::{self, CommandInput},
    common,
    datastore::{self, TimeSeriesData, TimeSeriesValue},
    models::{
        error::AppError,
        plugin::data::Data,
        plugin::monitoring::{KeyValue, Monitioring},
    },
    plugin_execution,
};

#[derive(Debug, Clone)]
struct IdentifiersAndValues {
    identifiers: HashMap<String, Vec<TimeSeriesValue>>,
    sub_identifiers: HashMap<String, Vec<TimeSeriesValue>>,
    values: HashMap<String, Vec<TimeSeriesValue>>,
}

impl IdentifiersAndValues {
    pub fn new() -> Self {
        IdentifiersAndValues {
            identifiers: HashMap::new(),
            sub_identifiers: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn value_names(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    pub fn max_value_list_length(&self) -> usize {
        let max_sub_length = self
            .sub_identifiers
            .values()
            .map(|v| v.len())
            .reduce(usize::max)
            .unwrap_or_default();

        let max_value_length = self
            .values
            .values()
            .map(|v| v.len())
            .reduce(usize::max)
            .unwrap_or_default();

        std::cmp::max(max_sub_length, max_value_length)
    }

    pub fn get_identifier(&self, name: String, index: usize) -> Option<TimeSeriesValue> {
        Self::get_at(Some(name), &self.identifiers, index)
    }

    pub fn get_sub_identifier(
        &self,
        name: Option<String>,
        index: usize,
    ) -> Option<TimeSeriesValue> {
        Self::get_at(name, &self.sub_identifiers, index)
    }

    pub fn get_value(&self, name: &str, index: usize) -> Option<TimeSeriesValue> {
        Self::get_at(Some(name.to_owned()), &self.values, index)
    }

    fn get_at(
        name: Option<String>,
        map: &HashMap<String, Vec<TimeSeriesValue>>,
        index: usize,
    ) -> Option<TimeSeriesValue> {
        let name = name?;

        if let Some(list) = map.get(name.as_str()) {
            if list.len() == 1 {
                list.iter().next().cloned()
            } else if list.len() > index {
                list.get(index).cloned()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn extract_values(
        mut self,
        json: Value,
        monitoring: &Monitioring,
        input: &CommandInput,
    ) -> Result<Self, AppError> {
        self.identifiers = get_values(
            &[monitoring.identifier.clone()],
            common::IDENTIFIER.to_owned(),
            &json,
            Some(input.clone()),
        )?;
        self.sub_identifiers = get_values(
            &monitoring.get_sub_identifiers_as_vec(),
            common::SUB_IDENTIFIER.to_owned(),
            &json,
            Some(input.clone()),
        )?;
        self.values = get_values(
            &monitoring.values,
            common::VALUE.to_owned(),
            &json,
            Some(input.clone()),
        )?;
        Ok(self)
    }
}

pub struct MonitoringDataExtractor {
    input_response_tuples: Vec<(CommandInput, String)>,
    data: Data,
}

impl MonitoringDataExtractor {
    pub fn new(responses: Vec<(CommandInput, String)>, data: &Data) -> Self {
        MonitoringDataExtractor {
            data: data.clone(),
            input_response_tuples: responses,
        }
    }

    pub fn parse(&self) -> Result<HashMap<String, Vec<TimeSeriesData>>, AppError> {
        log::trace!("result from monitoring is {:?}", self.input_response_tuples);

        let mut map = HashMap::new();

        for tuple in &self.input_response_tuples {
            for monitoring in &self.data.monitoring {
                let input = tuple.0.clone();
                let response = tuple.1.clone();

                let mut extracted_data =
                    extract_monitoring_data(response.as_str(), monitoring, &input)?;

                map.entry(monitoring.id.clone())
                    .or_insert_with(Vec::new)
                    .append(&mut extracted_data);
            }
        }
        Ok(map)
    }
}

fn extract_monitoring_data(
    response: &str,
    monitoring: &Monitioring,
    input: &CommandInput,
) -> Result<Vec<TimeSeriesData>, AppError> {
    let mut vec = Vec::new();

    let response = match &monitoring.pre_process {
        Some(script) => plugin_execution::pre_or_post_process(response, script)?,
        None => response.to_owned(),
    };

    match serde_json::from_str::<serde_json::Value>(response.as_str()) {
        Ok(json) => {
            let identifiers_and_values =
                IdentifiersAndValues::new().extract_values(json, monitoring, input)?;

            for value_name in identifiers_and_values.value_names() {
                for index in 0..identifiers_and_values.max_value_list_length() {
                    let identfier_name = &monitoring.identifier.name;
                    let sub_identifier_name =
                        monitoring.sub_identifier.as_ref().map(|kv| kv.name.clone());

                    let identifier =
                        identifiers_and_values.get_identifier(identfier_name.clone(), index);
                    let sub_identifier = identifiers_and_values
                        .get_sub_identifier(sub_identifier_name.clone(), index);
                    let value: Option<TimeSeriesValue> =
                        identifiers_and_values.get_value(value_name.as_str(), index);

                    if identifier.is_none() || value.is_none() {
                        continue;
                    }

                    let mut sub_identifiers: Vec<TimeSeriesValue> =
                        sub_identifier.iter().cloned().collect();

                    if identifiers_and_values.values.len() > 1 {
                        sub_identifiers.push(TimeSeriesValue::Symbol(
                            common::SUB_IDENTIFIER_2.to_owned(),
                            value_name.clone(),
                        ));
                    }

                    vec.push(TimeSeriesData {
                        identifier: identifier.ok_or(AppError::Unknown(
                            "Should acutally not happen since it is checked before".to_owned(),
                        ))?,
                        sub_identifiers,
                        value: value.ok_or(AppError::Unknown(
                            "Should acutally not happen since it is checked before".to_owned(),
                        ))?,
                        timestamp: datastore::Timestamp::SysTime(SystemTime::now()),
                    });
                }
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

fn get_values(
    fields: &[KeyValue],
    db_field_name: String,
    json: &serde_json::Value,
    input: Option<CommandInput>,
) -> Result<HashMap<String, Vec<TimeSeriesValue>>, AppError> {
    let mut map = HashMap::new();

    if fields.is_empty() {
        return Ok(map);
    }

    for field in fields {
        let field_name = field.name.clone();

        let replaced = replace_placeholders_in_values(&input, field.value.as_str());

        let string_values = match is_json_path_query(&replaced) {
            true => json_path_query(json, replaced.0.as_str())?,
            false => {
                log::debug!(
                    "value {} doesn't start with $. Treating it as a constant value",
                    replaced.0
                );
                vec![replaced.0]
            }
        };

        let values = string_values
            .iter()
            .flat_map(|s| make_value(field, db_field_name.clone(), s.as_str()))
            .collect();

        map.insert(field_name, values);
    }

    Ok(map)
}

fn replace_placeholders_in_values(input: &Option<CommandInput>, value: &str) -> (String, String) {
    match input {
        Some(input) => commands::replace(value, input),
        None => Ok((value.to_owned(), value.to_owned())),
    }
    .unwrap_or_default()
}

fn is_json_path_query(replaced: &(String, String)) -> bool {
    replaced.0.starts_with('$')
}

fn json_path_query(json: &Value, input: &str) -> Result<Vec<String>, AppError> {
    let found_values = json.clone().path(input)?;

    match found_values.is_array() {
        true => {
            let strings: Vec<String> = found_values
                .as_array()
                .ok_or(AppError::Unknown(format!(
                    "Could not get array from json value {}",
                    found_values
                )))?
                .iter()
                .flat_map(common::convert_value_to_str)
                .collect();

            Ok(strings)
        }
        false => Ok(vec![
            common::convert_value_to_str(&found_values).unwrap_or_default()
        ]),
    }
}

fn make_value(field: &KeyValue, name: String, value: &str) -> Option<TimeSeriesValue> {
    match field.value_type.as_str() {
        "symbol" => Some(TimeSeriesValue::Symbol(name, value.to_owned())),
        "boolean" => match value {
            "1" => Some(TimeSeriesValue::Bool(name, true)), // 1 is also traeted as true
            "0" => Some(TimeSeriesValue::Bool(name, false)), // 0 is traeted as false
            y => Some(TimeSeriesValue::Bool(
                name,
                y.to_lowercase().parse().unwrap_or_default(),
            )),
        },
        "integer" => Some(TimeSeriesValue::Int(
            name,
            value.parse().unwrap_or_default(),
        )),
        "float" => Some(TimeSeriesValue::Float(
            name,
            value.parse().unwrap_or_default(),
        )),
        "string" => Some(TimeSeriesValue::String(name, value.to_owned())),
        y => {
            log::error!("unknown field type {} found", y);
            None
        }
    }
}
