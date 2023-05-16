use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    ServerNotFound(String),
    InvalidArgument(String, Option<String>),
    UnknownPlugin(String),
    UnknownPluginAction(String, String),
    UnknownPluginData(String,String),
    Unknown(Box<dyn Error>),
    DatabaseError(Box<dyn Error>),
    MissingArgument(String),
    CouldNotRenderData(String)
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ServerNotFound(ipaddress) => write!(f, "A server with address {} could not be found", ipaddress),
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
            AppError::DatabaseError(err) => write!(f, "A database error occurred {}",err),
            AppError::MissingArgument(name) => write!(f, "Argument with name {} is missing or not set", name),            
            AppError::CouldNotRenderData(data) => write!(f, "Could not render data {}", data),

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


impl std::error::Error for AppError {}