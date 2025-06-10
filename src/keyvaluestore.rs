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

    pub fn exists(&self, key: &str) -> bool {
        match self.kv.get(key) {
            Some(_) => return true,
            None    => return false
        }
    }

    pub fn del(&mut self, key: &str) -> Result<String, &str> {
       match self.kv.remove(key) {
        Some(key ) => return Ok(key),
        None               => return Err("Key not found.")
       }
    } 
}