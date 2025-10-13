use std::collections::HashMap;

pub struct KeyMap {
    mappings: HashMap<String, String>,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, from: &str, to: &str) {
        self.mappings.insert(from.to_string(), to.to_string());
    }

    pub fn get_mapping(&self, key: &str) -> Option<&String> {
        self.mappings.get(key)
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}

