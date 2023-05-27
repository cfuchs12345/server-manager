use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
}

pub struct QueryParamsAsMap {
    params: HashMap<String, String>,
}

impl From<Vec<QueryParam>> for QueryParamsAsMap {
    fn from(input_params: Vec<QueryParam>) -> Self {
        QueryParamsAsMap {
            params: input_params
                .iter()
                .map(|param| (param.name.clone(), param.value.clone()))
                .collect(),
        }
    }
}

impl QueryParamsAsMap {
    pub fn get(&self, param: &str) -> Option<&String> {
        self.params.get(param)
    }

    pub fn get_as_str(&self, param: &str) -> Option<&str> {
        self.params.get(param).map(|value| value.as_str())
    }

    pub fn get_split_by(&self, param: &str, split: &str) -> Option<Vec<String>> {
        match self.params.get(param) {
            Some(value) => {
                let res: Vec<String> = value.split(split).map(str::to_string).collect();
                Some(res)
            }
            None => None,
        }
    }
}
