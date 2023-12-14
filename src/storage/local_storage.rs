use std::sync::Arc;
use std::sync::RwLock;

use crate::Result;
use crate::domain::Log;
use crate::usecase::Storage;

#[derive(Clone, Debug, Default)]
pub(crate) struct LocalStorage {
    pub(crate) data: Arc<RwLock<Vec<Log>>>,
}

impl LocalStorage {
    pub(crate) fn new() -> Self {
        LocalStorage {
            data: Arc::default(),
        }
    }
}

impl Storage for LocalStorage {
    fn save(&self, log: Log) -> Result<()> {
        let mut store = self.data.write().unwrap();
        store.push(log);
        return Ok(());
    }

    fn delete(&self, _log: Log) {
        todo!();
    }

    fn find<F>(&self, predicate: F) -> Vec<Log>
    where
        F: Fn(&Log) -> bool,
    {
        let store = self.data.read().unwrap();
        return store.iter().filter_map(|log| { if predicate(log) { Some(log.clone()) } else { None } }).collect();
    }
}
