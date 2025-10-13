use crate::buffer::Buffer;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
    pub desired_col: usize, // For vertical movement
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            desired_col: 0,
        }
    }

    pub fn move_left(&mut self, _buffer: &Buffer) {
        if self.col > 0 {
            self.col -= 1;
            self.desired_col = self.col;
        }
    }

    pub fn move_right(&mut self, buffer: &Buffer) {
        if let Some(line) = buffer.get_line(self.line) {
            if self.col < line.len() {
                self.col += 1;
                self.desired_col = self.col;
            }
        }
    }

    pub fn move_up(&mut self, buffer: &Buffer) {
        if self.line > 0 {
            self.line -= 1;
            self.clamp_col(buffer);
            self.col = self.desired_col.min(buffer.get_line(self.line).map(|l| l.len()).unwrap_or(0));
        }
    }

    pub fn move_down(&mut self, buffer: &Buffer) {
        if self.line < buffer.line_count() - 1 {
            self.line += 1;
            self.clamp_col(buffer);
            self.col = self.desired_col.min(buffer.get_line(self.line).map(|l| l.len()).unwrap_or(0));
        }
    }

    pub fn move_line_start(&mut self) {
        self.col = 0;
        self.desired_col = 0;
    }

    pub fn move_line_end(&mut self, buffer: &Buffer) {
        if let Some(line) = buffer.get_line(self.line) {
            self.col = line.len();
            self.desired_col = self.col;
        }
    }

    pub fn move_word_forward(&mut self, buffer: &Buffer) {
        if let Some(line) = buffer.get_line(self.line) {
            let chars: Vec<char> = line.chars().collect();
            let mut pos = self.col;

            // Skip current word
            while pos < chars.len() && !chars[pos].is_whitespace() {
                pos += 1;
            }
            // Skip whitespace
            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }

            self.col = pos;
            self.desired_col = pos;
        }
    }

    pub fn move_word_backward(&mut self, buffer: &Buffer) {
        if let Some(line) = buffer.get_line(self.line) {
            if self.col == 0 {
                return;
            }

            let chars: Vec<char> = line.chars().collect();
            let mut pos = self.col - 1;

            // Skip whitespace
            while pos > 0 && chars[pos].is_whitespace() {
                pos -= 1;
            }
            // Skip word
            while pos > 0 && !chars[pos - 1].is_whitespace() {
                pos -= 1;
            }

            self.col = pos;
            self.desired_col = pos;
        }
    }

    pub fn move_to_line(&mut self, line: usize, buffer: &Buffer) {
        self.line = line.min(buffer.line_count().saturating_sub(1));
        self.clamp_col(buffer);
    }

    fn clamp_col(&mut self, buffer: &Buffer) {
        if let Some(line) = buffer.get_line(self.line) {
            self.col = self.col.min(line.len());
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

