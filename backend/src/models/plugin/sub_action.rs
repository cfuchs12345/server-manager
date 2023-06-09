use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubAction {
    pub feature_id: Option<String>,
    pub action_id: Option<String>,
    #[serde(default)]
    pub action_params: Option<String>,
    #[serde(default)]
    pub action_image: Option<String>,
}

impl TryFrom<String> for SubAction {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let stripped_value = value
            .replace("[[Action ", "")
            .replace("]]", "")
            .replace('\"', "");

        let map: HashMap<String, String> = map_data(stripped_value)?;

        Ok(SubAction {
            feature_id: map.get("feature.id").map(|v| v.to_owned()),
            action_id: map.get("action.id").map(|v| v.to_owned()),
            action_params: map.get("action.params").map(|v| v.to_owned()),
            action_image: map.get("action.image").map(|v| v.to_owned()),
        })
    }
}

fn map_data(input: String) -> Result<HashMap<String, String>, AppError> {
    Ok(split_by_whitespace_and_equals(input)?
        .iter()
        .map(|(key, val)| (key, &val[1..]))
        .map(|(key, val)| (key.to_owned(), val.to_owned()))
        .collect())
}

fn split_by_whitespace_and_equals(input: String) -> Result<Vec<(String, String)>, AppError> {
    input
        .split_whitespace()
        .map(|s| split_in_two_pieces(s, '='))
        .collect()
}

fn split_in_two_pieces(to_split: &str, split_by: char) -> Result<(String, String), AppError> {
    let index = to_split.find(split_by).ok_or(AppError::Unknown(format!(
        "Expected an equal sign in {}",
        to_split
    )))?;

    let split = to_split.split_at(index);
    Ok((split.0.to_owned(), split.1.to_owned()))
}
