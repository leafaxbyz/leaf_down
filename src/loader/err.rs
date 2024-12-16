use std::error::Error;
use std::fmt::{Debug, Display};
use std::{fmt, io};

#[derive(Debug)]
pub enum CustomError {
    Err(String),
    ConfigReadErr(io::Error),
    ConfigParseErr(serde_json::error::Error),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Err(config) => write!(f, "File not found: {}", config),
            CustomError::ConfigReadErr(e) => write!(f, "IO error: {}", e),
            CustomError::ConfigParseErr(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for CustomError {}
