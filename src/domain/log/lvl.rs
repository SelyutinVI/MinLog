use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) enum Level {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Level::Error),
            "warning" => Ok(Level::Warning),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            "trace" => Ok(Level::Trace),
            _ => Err(Error::InvalidLevel),
        }
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::Error => "error".to_string(),
            Level::Warning => "warning".to_string(),
            Level::Info => "info".to_string(),
            Level::Debug => "debug".to_string(),
            Level::Trace => "trace".to_string(),
        }
    }
}
