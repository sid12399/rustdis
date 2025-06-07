use std::collections::HashMap;


pub struct KeyValueStore {
    kv: HashMap<String, String>
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore { kv: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.kv.get(key).map(|s| s.as_str())
    }

    pub fn insert(&mut self, key: String, val: String) {
        self.kv.insert(key, val);
    }
}