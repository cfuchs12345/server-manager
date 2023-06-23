use std::fmt::Display;

use serde::Serialize;

use crate::common;

#[derive(Debug)]
pub enum Error {
    UnexpectedExecution(std::io::Error),
    Communication(kafka::Error),
    Serialization(serde_json::Error),
    StringConversion(std::string::FromUtf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedExecution(err) => {
                write!(f, "Error while executing command: {}", err)
            }
            Self::Communication(err) => {
                write!(f, "Error while trying to communicate with Kafka: {}", err)
            }
            Self::Serialization(err) => {
                write!(f, "Error whileserializing struct: {}", err)
            }
            Self::StringConversion(err) => {
                write!(f, "Error while converting string: {}", err)
            }
        }
    }
}

impl From<kafka::Error> for Error {
    fn from(err: kafka::Error) -> Self {
        Error::Communication(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::UnexpectedExecution(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::StringConversion(err)
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    client_id: String,
    error_message: String,
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        ErrorResponse {
            client_id: common::CLIENT_ID.clone(),
            error_message: format!("{}", err),
        }
    }
}
