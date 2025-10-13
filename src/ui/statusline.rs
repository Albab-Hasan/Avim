use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::Mode;

pub struct StatusLine {
    mode: Mode,
    file_path: Option<String>,
    modified: bool,
    line: usize,
    col: usize,
    total_lines: usize,
}

impl StatusLine {
    pub fn new(mode: &Mode, buffer: &Buffer, cursor: &Cursor) -> Self {
        Self {
            mode: *mode,
            file_path: buffer.file_path().map(|p| p.display().to_string()),
            modified: buffer.is_modified(),
            line: cursor.line + 1,
            col: cursor.col + 1,
            total_lines: buffer.line_count(),
        }
    }

    pub fn render(&self, width: usize) -> String {
        let left = format!(
            " {} {}{}",
            self.mode.name(),
            self.file_path.as_deref().unwrap_or("[No Name]"),
            if self.modified { " [+]" } else { "" }
        );

        let right = format!(" {}:{} {}/{} ", self.line, self.col, self.line, self.total_lines);

        let padding = width.saturating_sub(left.len() + right.len());
        format!("{}{}{}", left, " ".repeat(padding), right)
    }
}

