mod gap_buffer;
mod line;

use std::fs;
use std::io;
use std::path::PathBuf;
use crate::syntax::Highlighter;
use syntect::highlighting::Style;

pub use gap_buffer::GapBuffer;
pub use line::Line;

#[derive(Clone)]
pub struct Buffer {
    lines: Vec<String>,
    file_path: Option<PathBuf>,
    modified: bool,
    undo_stack: Vec<BufferState>,
    redo_stack: Vec<BufferState>,
    highlighter: Highlighter,
    syntax_name: Option<String>,
}

#[derive(Clone)]
pub struct BufferState {
    pub lines: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            file_path: None,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            highlighter: Highlighter::new(),
            syntax_name: None,
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let file_path = PathBuf::from(path);
        let highlighter = Highlighter::new();
        let syntax_name = highlighter.detect_syntax(&file_path);
        
        // Try to read the file, but if it doesn't exist, create a new buffer with the path
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // File doesn't exist, create new buffer with the path
                return Ok(Self {
                    lines: vec![String::new()],
                    file_path: Some(file_path),
                    modified: false,
                    undo_stack: Vec::new(),
                    redo_stack: Vec::new(),
                    highlighter,
                    syntax_name,
                });
            }
            Err(e) => return Err(e), // Other errors (permission, etc.)
        };

        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        Ok(Self {
            lines,
            file_path: Some(file_path),
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            highlighter,
            syntax_name,
        })
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let content = self.lines.join("\n");
            fs::write(path, content)?;
            self.modified = false;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No file path set",
            ))
        }
    }

    pub fn save_as(&mut self, path: &str) -> io::Result<()> {
        self.file_path = Some(PathBuf::from(path));
        self.save()
    }

    pub fn get_line(&self, idx: usize) -> Option<&String> {
        self.lines.get(idx)
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, line: usize, col: usize, ch: char) {
        if line < self.lines.len() {
            self.save_state(line, col);
            self.lines[line].insert(col, ch);
            self.modified = true;
        }
    }

    pub fn delete_char(&mut self, line: usize, col: usize) {
        if line < self.lines.len() && col < self.lines[line].len() {
            self.save_state(line, col);
            self.lines[line].remove(col);
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self, line: usize, col: usize) {
        if line < self.lines.len() {
            self.save_state(line, col);
            let rest = self.lines[line].split_off(col);
            self.lines.insert(line + 1, rest);
            self.modified = true;
        }
    }

    pub fn delete_line(&mut self, line: usize) -> Option<String> {
        if line < self.lines.len() && self.lines.len() > 1 {
            self.save_state(line, 0);
            self.modified = true;
            Some(self.lines.remove(line))
        } else if self.lines.len() == 1 {
            self.save_state(line, 0);
            let content = self.lines[0].clone();
            self.lines[0].clear();
            self.modified = true;
            Some(content)
        } else {
            None
        }
    }

    pub fn join_lines(&mut self, line: usize) {
        if line < self.lines.len() - 1 {
            self.save_state(line, self.lines[line].len());
            let next_line = self.lines.remove(line + 1);
            if !self.lines[line].is_empty() && !next_line.is_empty() {
                self.lines[line].push(' ');
            }
            self.lines[line].push_str(&next_line.trim_start());
            self.modified = true;
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn get_line_mut(&mut self, idx: usize) -> Option<&mut String> {
        self.lines.get_mut(idx)
    }

    fn save_state(&mut self, cursor_line: usize, cursor_col: usize) {
        const MAX_UNDO_STACK: usize = 100;
        
        let state = BufferState {
            lines: self.lines.clone(),
            cursor_line,
            cursor_col,
        };
        
        self.undo_stack.push(state);
        if self.undo_stack.len() > MAX_UNDO_STACK {
            self.undo_stack.remove(0);
        }
        
        // Clear redo stack on new action
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<(usize, usize)> {
        if let Some(state) = self.undo_stack.pop() {
            // Save current state to redo stack
            let current_state = BufferState {
                lines: self.lines.clone(),
                cursor_line: state.cursor_line,
                cursor_col: state.cursor_col,
            };
            self.redo_stack.push(current_state);
            
            // Restore previous state
            self.lines = state.lines;
            self.modified = true;
            Some((state.cursor_line, state.cursor_col))
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<(usize, usize)> {
        if let Some(state) = self.redo_stack.pop() {
            // Save current state to undo stack
            let current_state = BufferState {
                lines: self.lines.clone(),
                cursor_line: state.cursor_line,
                cursor_col: state.cursor_col,
            };
            self.undo_stack.push(current_state);
            
            // Restore redo state
            self.lines = state.lines;
            self.modified = true;
            Some((state.cursor_line, state.cursor_col))
        } else {
            None
        }
    }

    pub fn highlight_line(&self, line_idx: usize) -> Vec<(Style, String)> {
        if let Some(line) = self.get_line(line_idx) {
            if let Some(ref syntax_name) = self.syntax_name {
                let highlighted = self.highlighter.highlight_line(line, syntax_name);
                if highlighted.is_empty() {
                    // Fallback if highlighting returns empty
                    vec![(Style::default(), line.clone())]
                } else {
                    highlighted.into_iter()
                        .map(|(style, text)| (style, text.to_string()))
                        .collect()
                }
            } else {
                vec![(Style::default(), line.clone())]
            }
        } else {
            vec![(Style::default(), String::new())]
        }
    }
    
    pub fn syntax_name(&self) -> Option<&str> {
        self.syntax_name.as_deref()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

