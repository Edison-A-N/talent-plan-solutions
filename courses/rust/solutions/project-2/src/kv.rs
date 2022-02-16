use crate::{sstable, error::Error};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    result::Result as stdResult,
};

pub type Result<T> = stdResult<T, Error>;
pub struct KvStore {
    pub store_map: HashMap<String, String>,
    pub log_file: PathBuf,
    sstable: sstable::SSTable,
}

impl KvStore {
    pub fn new() -> KvStore {
        let tmp_log = PathBuf::new();
        KvStore {
            sstable: sstable::SSTable::new(tmp_log.clone()),
            store_map: HashMap::new(),
            log_file: tmp_log.clone(),
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let sstcolumn = sstable::SSTColumn::new(
            sstable::ActionMap::Set, key.clone(), value.clone());
        self.sstable.write_ahead(sstcolumn).expect("write ahead error");
        self.store_map.insert(key, value);
        return Ok(());
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let value = &self.store_map.get(&key);
        return match value {
            // None => Err(Error {
            //     msg: String::from("Key not found"),
            //     err_type: 1,
            // }),
            Some(data) => Ok(Some(data.to_string())),
            _ => Ok(None),
        };
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        let sstcolumn = sstable::SSTColumn::new(
            sstable::ActionMap::Remove, key.clone(), "".to_owned());
        self.sstable.write_ahead(sstcolumn).expect("write ahead error");

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
        let log_file = _dir.join(Path::new("log1.sst"));
        let mut kvstore = KvStore{
            log_file: log_file.clone(),
            store_map: HashMap::new(),
            sstable: sstable::SSTable::new(log_file.clone())
        };
        kvstore.load();
        Ok(kvstore)
    }

    fn load(&mut self) {
        self.sstable.load();
        self.store_map = self.sstable.build_kvstore().expect("build kvstore error");
    }
}
