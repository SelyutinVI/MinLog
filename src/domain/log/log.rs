use serde::{Deserialize, Serialize};

use super::{Lifetime, Level};
use crate::Result;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Log {
    msg: String,
    level: Level,
    lifetime: Lifetime,
    body: Option<HashMap<String, String>>,
}

impl Log {
    pub(crate) fn create_log(
        msg: impl Into<String>,
        level: &str,
        lifetime: &str,
        body: Option<HashMap<String, String>>,
    ) -> Result<Self> {
        let level = Level::from_str(level)?;
        let lifetime = Lifetime::from_str(lifetime)?;

        let new_log = Self {
            msg: msg.into(),
            level,
            lifetime,
            body,
        };

        return Ok(new_log);
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;

    #[test]
    fn test_create_log() {
        let body = HashMap::from([("key".to_string(), "value".to_string())]);
        let log = Log::create_log(
            "test",
            "debug",
            "xs",
            Some(body),
        );
        assert!(log.is_ok());

        let log = log.unwrap();
        assert!(log.body.unwrap()["key"] == "value".to_string());
    }

    #[test]
    fn test_create_log_invalid_level() {
        let log = Log::create_log(
            "test",
            "asd",
            "xs",
            None,
        );
        assert_eq!(log.err().unwrap(), Error::InvalidLevel);
    }

    #[test]
    fn test_create_log_lifetime_level() {
        let log = Log::create_log(
            "test",
            "debug",
            "xssss",
            None,
        );
        assert_eq!(log.err().unwrap(), Error::InvalidLifetime);
    }
}
