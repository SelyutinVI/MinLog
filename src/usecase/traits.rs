use std::collections::HashMap;

use crate::domain::Log;
use crate::Result;

pub trait Logger {
    fn log(
        &self,
        msg: impl Into<String> + Send,
        level: &str,
        lifetime: &str,
        body: Option<HashMap<String, String>>,
    ) -> Result<()>;

    fn find_logs<F>(&self, predicate: F) -> Vec<Log>
    where
        F: Fn(&Log) -> bool + Send + 'static;

    fn cleanup(&self);
}

pub trait Storage {
    fn save(&self, log: Log) -> Result<()>;
    fn find<F>(&self, func: F) -> Vec<Log>
    where
        F: Fn(&Log) -> bool;
    fn delete(&self, log: Vec<Log>);
}
