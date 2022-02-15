use serde::{Serialize,Deserialize };
use serde_json;

use std::error::Error;
use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct SSTable {
    log_file: PathBuf,
    columns: Vec<SSTColumn>,
    serial_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionMap {
    Set,
    Remove,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SSTColumn {
    id: usize,
    pub action: ActionMap,
    pub key: String,
    pub value: String,
}

impl SSTColumn {
    pub fn new(action: ActionMap, key: String, value: String) -> SSTColumn {
        SSTColumn {
            id: 0, action: action, key: key, value: value
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
            serial_id: 0,
            log_file: log_file,
            columns: Vec::new(),
        }
    }
    pub fn write_ahead(&mut self, mut sstcolumn: SSTColumn) -> Result<(), Box<dyn Error>> {
        sstcolumn.id = self.serial_id;
        self.serial_id = self.serial_id + 1;
        self.load();
        self.columns.push(sstcolumn);
        self.write_to_file()?;
        Ok(())
    }
    pub fn load(&mut self) {
        match self.read_from_file() {
            Ok(columns) => {
                self.columns = columns;
                self.serial_id = self.columns.len();
            },
            Err(_) => ()
        }
    }
    
    fn write_to_file(&self) -> Result<(), Box<dyn Error>> {

        let json: String = serde_json::to_string(&self.columns)?;

        fs::write(&self.log_file, &json).expect("Unable to write file");

        println!("{}", &json);
        Ok(())
    }


    fn read_from_file(&self) -> Result<Vec<SSTColumn>, Box<dyn Error>> {
        let file = fs::File::open(&self.log_file)?;
        let reader = io::BufReader::new(file);
        let store_data = serde_json::from_reader(reader)?;
        Ok(store_data)
    }
}
