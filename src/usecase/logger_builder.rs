use super::{JustLogger, Storage};
use crate::{domain::Level, storage::LocalStorage};

pub struct HasStorage<S: Storage>(S);
pub struct NoStorage;

pub struct LoggerBuilder<S> {
    storage: S,
    min_level: Option<Level>,
}

impl LoggerBuilder<NoStorage> {
    pub fn new() -> Self {
        LoggerBuilder {
            storage: NoStorage,
            min_level: None,
        }
    }

    pub fn use_local_storage(self) -> LoggerBuilder<HasStorage<LocalStorage>> {
        let storage = LocalStorage::new();
        LoggerBuilder {
            storage: HasStorage(storage),
            min_level: self.min_level,
        }
    }
}

impl<T: Storage> LoggerBuilder<HasStorage<T>> {
    pub fn build_just_logger(self) -> JustLogger<T> {
        JustLogger::new(self.storage.0, self.min_level)
    }
}

impl<T> LoggerBuilder<T> {
    pub fn set_min_level(mut self, lvl: Level) -> Self {
        self.min_level = Some(lvl);
        return self;
    }
}
