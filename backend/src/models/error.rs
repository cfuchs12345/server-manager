use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    InvalidArgument(String, Option<String>),
    UnknownPlugin(String),
    UnknownPluginAction(String, String),
    UnknownPluginData(String,String),
    Unknown(Box<dyn Error>),
    MissingArgument(String),
}

impl AppError {
    

}