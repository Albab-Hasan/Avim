use std::path::Path;

pub struct Highlighter {
    language: Option<String>,
}

impl Highlighter {
    pub fn new() -> Self {
        Self { language: None }
    }

    pub fn detect_language(&mut self, file_path: &Path) {
        if let Some(ext) = file_path.extension() {
            self.language = match ext.to_str() {
                Some("rs") => Some("rust".to_string()),
                Some("py") => Some("python".to_string()),
                Some("js") => Some("javascript".to_string()),
                Some("cpp") | Some("cc") | Some("cxx") => Some("cpp".to_string()),
                Some("c") => Some("c".to_string()),
                Some("h") | Some("hpp") => Some("cpp".to_string()),
                _ => None,
            };
        }
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

