use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::Mode;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualType {
    Character,
    Line,
    Block,
}

pub struct VisualMode {
    visual_type: VisualType,
    start_line: usize,
    start_col: usize,
}

impl VisualMode {
    pub fn new(visual_type: VisualType, cursor: &Cursor) -> Self {
        Self {
            visual_type,
            start_line: cursor.line,
            start_col: cursor.col,
        }
    }

    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        cursor: &mut Cursor,
        buffer: &mut Buffer,
    ) -> Option<Mode> {
        match key.code {
            KeyCode::Esc => {
                return Some(Mode::Normal);
            }
            KeyCode::Char('h') => cursor.move_left(buffer),
            KeyCode::Char('j') => cursor.move_down(buffer),
            KeyCode::Char('k') => cursor.move_up(buffer),
            KeyCode::Char('l') => cursor.move_right(buffer),
            KeyCode::Char('w') => cursor.move_word_forward(buffer),
            KeyCode::Char('b') => cursor.move_word_backward(buffer),
            KeyCode::Char('0') => cursor.move_line_start(),
            KeyCode::Char('$') => cursor.move_line_end(buffer),
            KeyCode::Char('d') | KeyCode::Char('x') => {
                // Delete selection
                self.delete_selection(cursor, buffer);
                return Some(Mode::Normal);
            }
            KeyCode::Char('y') => {
                // Yank selection (to be implemented with register system)
                return Some(Mode::Normal);
            }
            _ => {}
        }
        None
    }

    fn delete_selection(&self, cursor: &mut Cursor, buffer: &mut Buffer) {
        let (start_line, end_line) = if self.start_line <= cursor.line {
            (self.start_line, cursor.line)
        } else {
            (cursor.line, self.start_line)
        };

        match self.visual_type {
            VisualType::Line => {
                for _ in start_line..=end_line {
                    buffer.delete_line(start_line);
                }
                cursor.line = start_line.min(buffer.line_count().saturating_sub(1));
                cursor.col = 0;
            }
            VisualType::Character => {
                // Simplified character-wise deletion
                if start_line == end_line {
                    let (start_col, end_col) = if self.start_col <= cursor.col {
                        (self.start_col, cursor.col)
                    } else {
                        (cursor.col, self.start_col)
                    };
                    
                    if let Some(line) = buffer.get_line_mut(start_line) {
                        line.drain(start_col..=end_col.min(line.len().saturating_sub(1)));
                    }
                    cursor.col = start_col;
                }
            }
            VisualType::Block => {
                // Block mode to be implemented
            }
        }
    }

    pub fn get_selection(&self, cursor: &Cursor) -> (usize, usize, usize, usize) {
        let (start_line, end_line) = if self.start_line <= cursor.line {
            (self.start_line, cursor.line)
        } else {
            (cursor.line, self.start_line)
        };

        let (start_col, end_col) = if self.start_col <= cursor.col {
            (self.start_col, cursor.col)
        } else {
            (cursor.col, self.start_col)
        };

        (start_line, start_col, end_line, end_col)
    }

    pub fn visual_type(&self) -> VisualType {
        self.visual_type
    }
}

