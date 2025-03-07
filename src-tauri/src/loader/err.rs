use crate::loader::err::CustomError::ConfigReadErr;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::{fmt, io};

#[derive(Debug)]
pub enum CustomError {
    Err(String),
    RequestError(reqwest::Error),
    ConfigReadErr(io::Error),
    ConfigParseErr(serde_json::error::Error),
}
impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Err(config) => write!(f, "File not found: {}", config),
            CustomError::ConfigReadErr(e) => write!(f, "IO error: {}", e),
            CustomError::ConfigParseErr(e) => write!(f, "Parse error: {}", e),
            CustomError::RequestError(e) => write!(f, "Request error: {}", e),
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(value: reqwest::Error) -> Self {
        CustomError::RequestError(value)
    }
}

impl From<io::Error> for CustomError {
    fn from(value: io::Error) -> Self {
        ConfigReadErr(value)
    }
}

impl Error for CustomError {}
