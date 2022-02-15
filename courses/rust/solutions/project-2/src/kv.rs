use crate::error::Error;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    result::Result as stdResult,
};

pub type Result<T> = stdResult<T, Error>;
pub struct KvStore {
    pub store_map: HashMap<String, String>,
    pub log_file: PathBuf,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store_map: HashMap::new(),
            log_file: PathBuf::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store_map.insert(key, value);
        return Ok(());
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let value = &self.store_map.get(&key);
        return match value {
            None => Err(Error {
                msg: String::from("Key not found"),
                err_type: 1,
            }),
            Some(data) => Result::Ok(Some(data.to_string())),
        };
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        let value = self.store_map.remove(&key);
        return match value {
            None => Err(Error {
                msg: String::from("Key not found"),
                err_type: 1,
            }),
            Some(_) => Result::Ok(()),
        };
    }
    pub fn open(_dir: &Path) -> Result<KvStore> {
        return Ok(KvStore::new());
    }
}
