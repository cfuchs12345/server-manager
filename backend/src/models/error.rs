use actix_web::{HttpResponse, ResponseError};
use http::{header, StatusCode};
use serde::{Deserialize, Serialize};
use std::str::ParseBoolError;
use std::{fmt::Display, net::AddrParseError, num::ParseIntError};
use surge_ping::SurgeError;

#[derive(Debug)]
pub enum AppError {
    ServerNotFound(String),
    UserNotFound(String),
    InvalidArgument(String, Option<String>),
    ArgumentNotFound(String),
    CredentialNotFound(String),
    CommandNotFound(String),
    UnknownPlugin(String),
    UnknownPluginAction(String, String),
    #[allow(dead_code)]
    UnknownPluginData(String, String),
    Unknown(String),
    DataNotFound(String),
    DatabaseError(String),
    HttpError(String),
    MissingArgument(String),
    CouldNotRenderData(String),
    UnAuthorized,
    DecryptionError,
    ParseError(String),
    EmailConfigError(String),
    Suppressed(String),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppErrorResponse {
    error: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ServerNotFound(ipaddress) => {
                write!(f, "A server with address {} could not be found", ipaddress)
            }
            AppError::UserNotFound(user_id) => {
                write!(f, "A user with id {} could not be found", user_id)
            }
            AppError::InvalidArgument(name, opt_value) => match opt_value {
                Some(value) => write!(f, "Invalid Argument {} with value {}", name, value),
                None => write!(f, "Invalid Argument {}", name),
            },
            AppError::ArgumentNotFound(name) => {
                write!(f, "Argument with name {} could not be found", name)
            }
            AppError::CredentialNotFound(name) => {
                write!(f, "Credential with name {} could not be found", name)
            }
            AppError::CommandNotFound(name) => {
                write!(f, "Command with name {} could not be found", name)
            }
            AppError::UnknownPlugin(name) => write!(f, "A plugin with id {} is not known", name),
            AppError::UnknownPluginAction(plugin_name, name) => write!(
                f,
                "A plugin action with id {} is not know for a plugin with id {}",
                name, plugin_name
            ),
            AppError::UnknownPluginData(plugin_name, name) => write!(
                f,
                "A plugin data query with id {} is not know for a plugin with id {}",
                name, plugin_name
            ),
            AppError::Unknown(err) => write!(f, "An unknown error occurred {}", err),
            AppError::DataNotFound(name) => write!(f, "data {} not found", name),
            AppError::DatabaseError(err) => write!(f, "A database error occurred {}", err),
            AppError::HttpError(err) => write!(f, "A http request error occurred {}", err),
            AppError::MissingArgument(name) => {
                write!(f, "Argument with name {} is missing or not set", name)
            }
            AppError::CouldNotRenderData(data) => write!(f, "Could not render data {}", data),
            AppError::UnAuthorized => write!(f, "User is not authorized"),
            AppError::DecryptionError => write!(f, "Data could not be decrypted"),
            AppError::ParseError(err) => write!(f, "Could not parse given data {}", err),
            AppError::EmailConfigError(name) => write!(
                f,
                "the email config in the env file is invalid for property: {}",
                name
            ),
            AppError::Suppressed(err) => write!(f, "Explicitly suppressed error {}", err),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(format!("{}", err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(format!("{}", err))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<header::ToStrError> for AppError {
    fn from(err: header::ToStrError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseError(format!("{}", err))
    }
}

impl From<ParseBoolError> for AppError {
    fn from(err: ParseBoolError) -> Self {
        AppError::ParseError(format!("{}", err))
    }
}

impl From<AddrParseError> for AppError {
    fn from(err: AddrParseError) -> Self {
        AppError::ParseError(format!("{}", err))
    }
}

impl From<SurgeError> for AppError {
    fn from(err: SurgeError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnAuthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: format!("{:?}", self),
        })
    }
}

impl std::error::Error for AppError {}
