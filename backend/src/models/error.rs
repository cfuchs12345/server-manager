use std::{error::Error, fmt::Display, num::ParseIntError};

use actix_web::{ResponseError, HttpResponse};
use http::{StatusCode, header};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AppError {
    ServerNotFound(String),
    UserNotFound(String),
    InvalidArgument(String, Option<String>),
    UnknownPlugin(String),
    UnknownPluginAction(String, String),
    #[allow(dead_code)]
    UnknownPluginData(String,String),
    Unknown(Box<dyn Error>),
    DataNotFound(String),
    DatabaseError(Box<dyn Error>),
    MissingArgument(String),
    CouldNotRenderData(String),
    UnAuthorized,
    DecryptionError,
    ParseError(Box<dyn Error>),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppErrorResponse {
    error: String
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ServerNotFound(ipaddress) => write!(f, "A server with address {} could not be found", ipaddress),
            AppError::UserNotFound(user_id) => write!(f, "A user with id {} could not be found", user_id),
            AppError::InvalidArgument(name, opt_value) => match opt_value {
                Some(value) => 
                write!(f, "Invalid Argument {} with value {}", name, value)
                ,
                None => 
                    write!(f, "Invalid Argument {}", name)
            },
            AppError::UnknownPlugin(name) => write!(f, "A plugin with id {} is not known", name),
            AppError::UnknownPluginAction(plugin_name, name) => write!(f, "A plugin action with id {} is not know for a plugin with id {}", name, plugin_name),
            AppError::UnknownPluginData(plugin_name, name) => write!(f, "A plugin data query with id {} is not know for a plugin with id {}", name, plugin_name),
            AppError::Unknown(err) => write!(f, "An unknown error occurred {}",err),
            AppError::DataNotFound(name) =>write!(f, "data {} not found", name),
            AppError::DatabaseError(err) => write!(f, "A database error occurred {}",err),
            AppError::MissingArgument(name) => write!(f, "Argument with name {} is missing or not set", name),            
            AppError::CouldNotRenderData(data) => write!(f, "Could not render data {}", data),
            AppError::UnAuthorized => write!(f, "User is not authorized"),
            AppError::DecryptionError => write!(f, "Data could not be decrypted"),
            AppError::ParseError(err) => write!(f, "Could not parse given data {}",err),

        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Unknown(Box::new(err))
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Unknown(Box::new(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(Box::new(err))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Unknown(Box::new(err))
    }
}


impl From<header::ToStrError> for AppError {
    fn from(err: header::ToStrError) -> Self {
        AppError::Unknown(Box::new(err))
    }
}


impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Unknown(Box::new(err))
    }
}


impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnAuthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: format!("{:?}", self),
        })
    }
}

impl std::error::Error for AppError {}