use std::str::FromStr;

use crate::{errors::Error, systeminfo};

pub enum NonSystemCommands {
    OsInformation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Unknown;

impl FromStr for NonSystemCommands {
    type Err = Unknown;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "os-information" => Ok(Self::OsInformation),
            _ => Err(Unknown),
        }
    }
}

impl NonSystemCommands {
    pub fn execute(&self) -> Result<Option<String>, Error> {
        match self {
            Self::OsInformation => {
                let si = systeminfo::get_system_info();

                Ok(Some(serde_json::to_string(&si).map_err(Error::from)?))
            }
        }
    }
}
