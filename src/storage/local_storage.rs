use std::sync::Arc;
use std::sync::RwLock;

use crate::domain::Log;
use crate::usecase::Storage;
use crate::Result;

#[derive(Clone, Debug, Default)]
pub struct LocalStorage {
    pub data: Arc<RwLock<Vec<Log>>>,
}

impl LocalStorage {
    pub fn new() -> Self {
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

    fn delete(&self, log: Vec<Log>) {
        let mut store = self.data.write().unwrap();
        store.retain(|l| !log.contains(l));
    }

    fn find<F>(&self, predicate: F) -> Vec<Log>
    where
        F: Fn(&Log) -> bool,
    {
        let store = self.data.read().unwrap();
        return store
            .iter()
            .filter_map(|log| {
                if predicate(log) {
                    Some(log.clone())
                } else {
                    None
                }
            })
            .collect();
    }
}
