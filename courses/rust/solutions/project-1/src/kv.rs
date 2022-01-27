use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    pub store_map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store_map: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store_map.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        let value = &self.store_map.get(&key);
        return match value {
            None => None,
            Some(data) => Some(data.to_string()),
        };
    }
    pub fn remove(&mut self, key: String) {
        self.store_map.remove(&key);
    }
}
