use kvs::log;
use kvs::log::LogProxy;
use std::{fs, path::PathBuf};

static TMP_LOG: &str = "/tmp/test_log_dir.sst";

fn clear() {
    fs::remove_file(PathBuf::from(&TMP_LOG)).expect("remove file error");
}

#[test]
fn new_sstable() {
    let _ = LogProxy::new(PathBuf::from(&TMP_LOG));
}

#[test]
fn load_sstable() {
    let mut log = LogProxy::new(PathBuf::from(&TMP_LOG));
    log.load();
}

#[test]
fn write_sstable() {
    clear();
    let mut log = LogProxy::new(PathBuf::from(&TMP_LOG));
    log.load();
    let logrow = log::LogRow::new(log::ActionMap::Set, "1".to_owned(), "1".to_owned());
    log.write_ahead(logrow).expect("write ahead error");
}
