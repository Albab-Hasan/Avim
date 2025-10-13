use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style, Theme};
use syntect::easy::HighlightLines;
use std::path::Path;

#[derive(Clone)]
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        
        Self { syntax_set, theme }
    }
    
    pub fn highlight_line<'a>(&self, line: &'a str, syntax_name: &str) -> Vec<(Style, &'a str)> {
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(syntax_name) {
            let mut h = HighlightLines::new(syntax, &self.theme);
            h.highlight_line(line, &self.syntax_set).unwrap_or_default()
        } else {
            vec![(Style::default(), line)]
        }
    }
    
    pub fn detect_syntax(&self, file_path: &Path) -> Option<String> {
        if let Some(syntax) = self.syntax_set.find_syntax_for_file(file_path).ok().flatten() {
            Some(syntax.name.clone())
        } else {
            None
        }
    }
    
    pub fn syntax_name(&self) -> Option<&str> {
        None // This will be set per buffer
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

