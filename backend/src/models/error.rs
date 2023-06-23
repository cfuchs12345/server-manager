use actix_web::{HttpResponse, ResponseError};
use base64::DecodeError;
use http::header::{InvalidHeaderName, InvalidHeaderValue};
use http::{header, StatusCode};
use lettre::address::AddressError;
use magic_crypt::MagicCryptError;
use serde::{Deserialize, Serialize};
use std::str::ParseBoolError;
use std::string::FromUtf8Error;
use std::sync::TryLockError;
use std::{fmt::Display, net::AddrParseError, num::ParseIntError};
use surge_ping::SurgeError;
use tokio::sync::AcquireError;
use tokio_cron_scheduler::JobSchedulerError;

#[allow(dead_code)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    InvalidPassword,
    DNSServersNotConfigured(),
    ServerNotFound(String),
    FeatureNotFound(String, String),
    UserNotFound(String),
    InvalidArgument(String, Option<String>),
    ArgumentNotFound(String),
    CredentialNotFound(String),
    CommandNotFound(String),
    CommunicationError(String),
    UnknownPlugin(String),
    UnknownPluginAction(String, String),
    UnknownPluginData(String, String),
    Unknown(String),
    DataNotFound(String),
    DatabaseError(String),
    HttpError(String),
    MissingArgument(String),
    MissingURLParameter(String),
    UnsupportedURLParameter(String, Option<String>),
    CouldNotRenderData(String),
    UnAuthorized,
    DecryptionError,
    ParseError(String),
    EmailConfigError(String),
    Suppressed(String),
    ScriptError(String),
    NokOKResponse(StatusCode, String),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppErrorResponse {
    error: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidPassword => {
                write!(f, "The password was invalid")
            }
            AppError::CommunicationError(err) => {
                write!(
                    f,
                    "An error occured while trying to communicate with another process: {}",
                    err
                )
            }
            AppError::DNSServersNotConfigured() => {
                write!(
                    f,
                    "There is no DNS Server configured but DNS lookup is activated. Cannot continue."
                )
            }
            AppError::ServerNotFound(ipaddress) => {
                write!(
                    f,
                    "A server with address '{}' could not be found",
                    ipaddress
                )
            }
            AppError::FeatureNotFound(ipaddress, feature) => {
                write!(
                    f,
                    "A server with address '{}' doesn't have a feature with id '{}'",
                    ipaddress, feature
                )
            }
            AppError::UserNotFound(user_id) => {
                write!(f, "A user with id '{}' could not be found", user_id)
            }
            AppError::InvalidArgument(name, opt_value) => match opt_value {
                Some(value) => write!(f, "Invalid Argument {} with value {}", name, value),
                None => write!(f, "Invalid Argument '{}'", name),
            },
            AppError::ArgumentNotFound(name) => {
                write!(f, "Argument with name '{}' could not be found", name)
            }
            AppError::CredentialNotFound(name) => {
                write!(f, "Credential with name '{}' could not be found", name)
            }
            AppError::CommandNotFound(name) => {
                write!(f, "Command with name '{}' could not be found", name)
            }
            AppError::UnknownPlugin(name) => write!(f, "A plugin with id '{}' is not known", name),
            AppError::UnknownPluginAction(plugin_name, name) => write!(
                f,
                "A plugin action with id '{}' is not know for a plugin with id '{}'",
                name, plugin_name
            ),
            AppError::UnknownPluginData(plugin_name, name) => write!(
                f,
                "A plugin data query with id '{}' is not know for a plugin with id '{}'",
                name, plugin_name
            ),
            AppError::Unknown(err) => write!(f, "An unknown error occurred {}", err),
            AppError::DataNotFound(name) => write!(f, "data '{}' not found", name),
            AppError::DatabaseError(err) => write!(f, "A database error occurred {}", err),
            AppError::HttpError(err) => write!(f, "A http request error occurred {}", err),
            AppError::MissingArgument(name) => {
                write!(f, "Argument with name '{}' is missing or not set", name)
            }
            AppError::MissingURLParameter(name) => {
                write!(f, "URL Paramter with name '{}' was not set", name)
            }
            AppError::UnsupportedURLParameter(name, value) => match value {
                Some(value) => write!(
                    f,
                    "URL Parameter '{}' with with value '{}' is not supported",
                    name, value
                ),
                None => write!(f, "URL Parameter '{}' is not supported", name),
            },
            AppError::CouldNotRenderData(data) => write!(f, "Could not render data '{}'", data),
            AppError::UnAuthorized => write!(f, "User is not authorized"),
            AppError::DecryptionError => write!(f, "Data could not be decrypted"),
            AppError::ParseError(err) => write!(f, "Could not parse given data '{}'", err),
            AppError::EmailConfigError(name) => write!(
                f,
                "the email config in the env file is invalid for property: '{}'",
                name
            ),
            AppError::Suppressed(err) => write!(f, "Explicitly suppressed error '{}'", err),
            AppError::ScriptError(err) => write!(f, "Error during script execution '{}'", err),
            AppError::NokOKResponse(statuscode, response) => write!(
                f,
                "Response was no OK or ACCEPTED but {} and response '{}'",
                statuscode, response
            ),
        }
    }
}

impl AppError {
    fn get_level(&self) -> Level {
        match self {
            AppError::Suppressed(_err) => Level::Debug,
            _ => Level::Error,
        }
    }

    pub fn log(&self) {
        match self.get_level() {
            Level::Debug => log::debug!("{}", &self),
            Level::Info => log::info!("{}", &self),
            Level::Warn => log::warn!("{}", &self),
            Level::Error => log::error!("{}", &self),
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

impl<T> From<TryLockError<T>> for AppError {
    fn from(err: TryLockError<T>) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<tokio::sync::TryLockError> for AppError {
    fn from(err: tokio::sync::TryLockError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<MagicCryptError> for AppError {
    fn from(err: MagicCryptError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Unknown(err)
    }
}

impl From<InvalidHeaderName> for AppError {
    fn from(err: InvalidHeaderName) -> Self {
        AppError::InvalidArgument(format!("Invalid headername {}", err), None)
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(err: InvalidHeaderValue) -> Self {
        AppError::InvalidArgument(format!("Invalid header value {}", err), None)
    }
}

impl From<AddressError> for AppError {
    fn from(err: AddressError) -> Self {
        AppError::InvalidArgument(format!("Invalid email config {}", err), None)
    }
}

impl From<lettre::error::Error> for AppError {
    fn from(err: lettre::error::Error) -> Self {
        AppError::InvalidArgument(format!("could not build email {}", err), None)
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        AppError::InvalidArgument(format!("could not send email {}", err), None)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::InvalidArgument(format!("config is invalid {}", err), None)
    }
}

impl From<DecodeError> for AppError {
    fn from(err: DecodeError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(err: FromUtf8Error) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<AcquireError> for AppError {
    fn from(err: AcquireError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<JobSchedulerError> for AppError {
    fn from(err: JobSchedulerError) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::Unknown(format!("{}", err))
    }
}

impl From<kafka::Error> for AppError {
    fn from(err: kafka::Error) -> Self {
        AppError::CommunicationError(format!("{}", err))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnAuthorized | Self::InvalidPassword | Self::UserNotFound(_) => {
                StatusCode::UNAUTHORIZED
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: format!("{}", self),
        })
    }
}

//impl std::error::Error for AppError {}
