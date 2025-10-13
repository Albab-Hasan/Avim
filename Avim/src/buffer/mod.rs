mod gap_buffer;
mod line;

use std::fs;
use std::io;
use std::path::PathBuf;

pub use gap_buffer::GapBuffer;
pub use line::Line;

#[derive(Clone)]
pub struct Buffer {
    lines: Vec<String>,
    file_path: Option<PathBuf>,
    modified: bool,
    undo_stack: Vec<BufferState>,
    redo_stack: Vec<BufferState>,
}

#[derive(Clone)]
struct BufferState {
    lines: Vec<String>,
    cursor_line: usize,
    cursor_col: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            file_path: None,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        Ok(Self {
            lines,
            file_path: Some(PathBuf::from(path)),
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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
            self.lines[line].insert(col, ch);
            self.modified = true;
        }
    }

    pub fn delete_char(&mut self, line: usize, col: usize) {
        if line < self.lines.len() && col < self.lines[line].len() {
            self.lines[line].remove(col);
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self, line: usize, col: usize) {
        if line < self.lines.len() {
            let rest = self.lines[line].split_off(col);
            self.lines.insert(line + 1, rest);
            self.modified = true;
        }
    }

    pub fn delete_line(&mut self, line: usize) -> Option<String> {
        if line < self.lines.len() && self.lines.len() > 1 {
            self.modified = true;
            Some(self.lines.remove(line))
        } else if self.lines.len() == 1 {
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
            let next_line = self.lines.remove(line + 1);
            self.lines[line].push(' ');
            self.lines[line].push_str(&next_line);
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
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

