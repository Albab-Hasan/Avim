use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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

    fn key_to_string(key: &KeyEvent) -> String {
        let mut result = String::new();

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            result.push_str("C-");
        }
        if key.modifiers.contains(KeyModifiers::ALT) {
            result.push_str("A-");
        }
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            result.push_str("S-");
        }

        match key.code {
            KeyCode::Char(c) => result.push(c),
            KeyCode::Enter => result.push_str("Enter"),
            KeyCode::Esc => result.push_str("Esc"),
            KeyCode::Backspace => result.push_str("BS"),
            KeyCode::Tab => result.push_str("Tab"),
            _ => {}
        }

        result
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}

