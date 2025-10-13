use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub keybindings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_tab_size")]
    pub tab_size: usize,
    #[serde(default)]
    pub expand_tabs: bool,
    #[serde(default = "default_true")]
    pub auto_indent: bool,
    #[serde(default = "default_true")]
    pub line_numbers: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_color_scheme")]
    pub color_scheme: String,
    #[serde(default = "default_true")]
    pub show_status_line: bool,
}

fn default_tab_size() -> usize {
    4
}

fn default_true() -> bool {
    true
}

fn default_color_scheme() -> String {
    "default".to_string()
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_size: default_tab_size(),
            expand_tabs: false,
            auto_indent: true,
            line_numbers: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            color_scheme: default_color_scheme(),
            show_status_line: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig::default(),
            ui: UiConfig::default(),
            keybindings: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self, path: &PathBuf) -> io::Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, contents)
    }
}

