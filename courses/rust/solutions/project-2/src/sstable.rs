use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;
use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct SSTable {
    log_file: PathBuf,
    columns: Vec<SSTColumn>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionMap {
    Set,
    Remove,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SSTColumn {
    pub action: ActionMap,
    pub key: String,
    pub value: String,
}

impl SSTColumn {
    pub fn new(action: ActionMap, key: String, value: String) -> SSTColumn {
        SSTColumn {
            action: action,
            key: key,
            value: value,
        }
    }
}

impl SSTable {
    pub fn new(log_file: PathBuf) -> SSTable {
        if !fs::metadata(&log_file).is_ok() {
            fs::File::create(&log_file)
                .unwrap_or_else(|error| panic!("Failed to create the file: {:?}", error));
        }
        SSTable {
            log_file: log_file,
            columns: Vec::new(),
        }
    }
    pub fn write_ahead(&mut self, sstcolumn: SSTColumn) -> Result<(), Box<dyn Error>> {
        self.write_to_file(sstcolumn)?;
        Ok(())
    }
    pub fn load(&mut self) {
        match self.read_from_file() {
            Ok(_) => (),
            Err(e) => {
                println!("Unable to load: {:?}", e);
                ()
            }
        };
        self.compaction().expect("Unable to compaction");
    }

    pub fn build_kvstore(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut kvstore = HashMap::new();
        for sstcolumn in &self.columns {
            match sstcolumn.action {
                ActionMap::Set => kvstore.insert(sstcolumn.key.clone(), sstcolumn.value.clone()),
                ActionMap::Remove => kvstore.remove(&sstcolumn.key),
            };
        }
        Ok(kvstore)
    }

    fn write_to_file(&self, sstcolumn: SSTColumn) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new().append(true).open(&self.log_file)?;
        file.seek(io::SeekFrom::End(0))
            .expect("Unable to seek to log file");
        let buf = serde_json::to_vec(&sstcolumn).expect("Unable to serialize to log file");
        file.write(&buf)?;
        Ok(())
    }

    fn read_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        let file = fs::File::open(&self.log_file)?;
        let mut reader = io::BufReader::new(file);
        let mut buf = vec![];
        while let Ok(_) = reader.read_until(b'}', &mut buf) {
            if buf.is_empty() {
                break;
            }
            let column: SSTColumn = serde_json::from_slice(&buf)?;
            self.columns.push(column);
            buf.clear();
        }
        Ok(())
    }

    fn compaction(&mut self) -> Result<(), Box<dyn Error>> {
        // let tmp_log_file = "kvstore.tmp.log";
        let mut file = fs::File::create(&self.log_file)?;
        let kvstore = self.build_kvstore().expect("build kvstore error");
        for column in &self.columns {
            if let Some(value) = kvstore.get(&(column.key)) {
                if column.value.eq(value) {
                    let buf = serde_json::to_vec(&column)?;
                    file.write(&buf)?;
                }
            }
        }
        // fs::rename(&self.log_file, "aaa.txt")?;
        Ok(())
    }
}
