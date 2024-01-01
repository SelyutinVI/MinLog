use super::super::{Logger, Storage};
use crate::domain::{Level, Log};
use crate::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct JustLogger<T>
where
    T: Storage,
{
    storage: T,
    min_lvl: Option<Level>,
}

impl<T: Storage> JustLogger<T> {
    pub(crate) fn new(storage: T, min_lvl: Option<Level>) -> Self {
        JustLogger { storage, min_lvl }
    }
}

impl<T: Storage> Logger for JustLogger<T> {
    fn log(
        &self,
        msg: impl Into<String>,
        level: &str,
        lifetime: &str,
        body: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let log = Log::create_log(msg, level, lifetime, body)?;
        if self.min_lvl.is_none() || log.level >= *self.min_lvl.as_ref().unwrap() {
            self.storage.save(log)?;
        }

        return Ok(());
    }

    fn find_logs<F>(&self, predicate: F) -> Vec<Log>
    where
        F: Fn(&Log) -> bool + Send + 'static,
    {
        return self.storage.find(predicate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::LocalStorage;

    #[test]
    fn test_log() {
        let logger = JustLogger::new(LocalStorage::new(), Some(Level::Info));
        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 0);

        let result = logger.log("test", "Info", "xs", None);
        assert!(result.is_ok());

        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 1);
    }

    #[test]
    fn test_log_with_less_lvl() {
        let logger = JustLogger::new(LocalStorage::new(), Some(Level::Info));
        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 0);

        let result = logger.log("test", "debug", "xs", None);
        assert!(result.is_ok());

        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 0);
    }
}
