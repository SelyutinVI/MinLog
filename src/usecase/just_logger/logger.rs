use std::collections::HashMap;
use super::super::{Logger, Storage};
use crate::domain::Log;
use crate::Result;

#[derive(Debug, Clone)]
pub(crate) struct JustLogger<T>
where
    T: Storage,
{
    storage: T,
}

impl<T: Storage> JustLogger<T> {
    pub(crate) fn new(storage: T) -> Self {
        JustLogger { storage }
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
        self.storage.save(log)?;

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
    use crate::storage::LocalStorage;
    use super::*;

    #[tokio::test]
    async fn test() {
        let logger = JustLogger::new(LocalStorage::new());
        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 0);

        let result = logger
            .log(
                "test",
                "debug",
                "xs",
                None,
            );
        assert!(result.is_ok());

        let logs = logger.find_logs(move |_| true);
        assert!(logs.len() == 1);
    }
}
