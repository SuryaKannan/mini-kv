use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MiniKV {
    store: HashMap<String, String>,
}

impl MiniKV {
    pub fn new() -> Self {
        MiniKV {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}
