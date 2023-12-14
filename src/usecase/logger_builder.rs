use super::{JustLogger, Storage};
use crate::storage::LocalStorage;

pub(crate) struct HasStorage<S: Storage>(S);
pub(crate) struct NoStorage;

pub(crate) struct LoggerBuilder<S> {
    storage: S,
}

impl LoggerBuilder<NoStorage> {
    pub(crate) fn new() -> Self {
        LoggerBuilder { storage: NoStorage }
    }
}

impl LoggerBuilder<NoStorage> {
    pub(crate) fn use_local_storage(self) -> LoggerBuilder<HasStorage<LocalStorage>> {
        let storage = LocalStorage::new();
        LoggerBuilder {
            storage: HasStorage(storage),
        }
    }
}

impl<T: Storage> LoggerBuilder<HasStorage<T>> {
    pub(crate) fn build_just_logger(self) -> JustLogger<T> {
        JustLogger::new(self.storage.0)
    }
}
