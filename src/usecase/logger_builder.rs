use super::{JustLogger, Storage};
use crate::{storage::LocalStorage, domain::Level};

pub(crate) struct HasStorage<S: Storage>(S);
pub(crate) struct NoStorage;

pub(crate) struct LoggerBuilder<S> {
    storage: S,
    min_level: Option<Level>,
}

impl LoggerBuilder<NoStorage> {
    pub(crate) fn new() -> Self {
        LoggerBuilder { storage: NoStorage, min_level: None }
    }

    pub(crate) fn use_local_storage(self) -> LoggerBuilder<HasStorage<LocalStorage>> {
        let storage = LocalStorage::new();
        LoggerBuilder {
            storage: HasStorage(storage),
            min_level: self.min_level,
        }
    }
}

impl<T: Storage> LoggerBuilder<HasStorage<T>> {
    pub(crate) fn build_just_logger(self) -> JustLogger<T> {
        JustLogger::new(self.storage.0, self.min_level)
    }
}


impl<T> LoggerBuilder<T> {
    pub(crate) fn set_min_level(mut self, lvl: Level) -> Self {
        self.min_level = Some(lvl);
        return self;
    }
}