use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Level {
    Debug,
    Trace,
    Info,
    Warning,
    Error,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_to_string() {
        assert_eq!(Level::Error.to_string(), "error");
        assert_eq!(Level::Warning.to_string(), "warning");
        assert_eq!(Level::Info.to_string(), "info");
        assert_eq!(Level::Debug.to_string(), "debug");
        assert_eq!(Level::Trace.to_string(), "trace");
    }

    #[test]
    fn test_create_level_from_str() {
        assert_eq!(Level::from_str("error").unwrap(), Level::Error);
        assert_eq!(Level::from_str("warning").unwrap(), Level::Warning);
        assert_eq!(Level::from_str("info").unwrap(), Level::Info);
        assert_eq!(Level::from_str("debug").unwrap(), Level::Debug);
        assert_eq!(Level::from_str("trace").unwrap(), Level::Trace);

        assert_eq!(Level::from_str("unknown"), Err(Error::InvalidLevel));
    }

    #[test]
    fn test_ord_level() {
        assert!(Level::Warning < Level::Error);
        assert!(Level::Info < Level::Warning);
        assert!(Level::Trace < Level::Info);
        assert!(Level::Debug < Level::Trace);
    }
}
