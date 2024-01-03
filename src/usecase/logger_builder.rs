use super::{CommonLogger, Storage};
use crate::domain::Level;

pub struct HasStorage<S: Storage>(S);
pub struct NoStorage;

pub struct LoggerBuilder<S> {
    storage: S,
    min_level: Option<Level>,
}

impl<T> LoggerBuilder<T> {
    pub fn use_min_level(mut self, lvl: Level) -> Self {
        self.min_level = Some(lvl);
        return self;
    }
}

impl LoggerBuilder<NoStorage> {
    pub fn new() -> LoggerBuilder<NoStorage> {
        LoggerBuilder {
            storage: NoStorage,
            min_level: None,
        }
    }

    pub fn use_storage<T: Storage>(self, storage: T) -> LoggerBuilder<HasStorage<T>> {
        LoggerBuilder {
            storage: HasStorage(storage),
            min_level: self.min_level,
        }
    }
}

impl<T: Storage> LoggerBuilder<HasStorage<T>> {
    pub fn build_common_logger(self) -> CommonLogger<T> {
        CommonLogger::new(self.storage.0, self.min_level)
    }
}
