use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SystemInformation {
    pub memory_stats: Vec<SystemInformationEntry>,
    pub memory_usage: Vec<SystemInformationEntry>,
    pub load_average: Vec<SystemInformationEntry>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SystemInformationEntry {
    pub name: String,
    pub value: f64,
}

impl SystemInformationEntry {
    pub fn new_usize(name: &str, value: usize) -> Self {
        SystemInformationEntry {
            name: name.to_owned(),
            value: value as f64,
        }
    }

    pub fn new_u64(name: &str, value: u64) -> Self {
        SystemInformationEntry {
            name: name.to_owned(),
            value: value as f64,
        }
    }

    pub fn new(name: &str, value: f64) -> Self {
        SystemInformationEntry {
            name: name.to_owned(),
            value,
        }
    }
}
