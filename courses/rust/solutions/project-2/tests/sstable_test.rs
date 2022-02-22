use kvs::sstable;
use kvs::sstable::SSTable;
use std::{fs, path::PathBuf};

static TMP_LOG: &str = "/tmp/test_log_dir.sst";

fn clear() {
    fs::remove_file(PathBuf::from(&TMP_LOG)).expect("remove file error");
}

#[test]
fn new_sstable() {
    let _ = SSTable::new(PathBuf::from(&TMP_LOG));
}

#[test]
fn load_sstable() {
    let mut sstable = SSTable::new(PathBuf::from(&TMP_LOG));
    sstable.load();
}

#[test]
fn write_sstable() {
    clear();
    let mut sstable = SSTable::new(PathBuf::from(&TMP_LOG));
    sstable.load();
    let sstcolumn =
        sstable::SSTColumn::new(sstable::ActionMap::Set, "1".to_owned(), "1".to_owned());
    sstable.write_ahead(sstcolumn).expect("write ahead error");
}
