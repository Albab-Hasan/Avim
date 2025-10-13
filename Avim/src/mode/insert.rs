use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::Mode;
use crossterm::event::{KeyCode, KeyEvent};

pub struct InsertMode;

impl InsertMode {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_key(
        key: KeyEvent,
        cursor: &mut Cursor,
        buffer: &mut Buffer,
    ) -> Option<Mode> {
        match key.code {
            KeyCode::Esc => {
                // Move cursor left when exiting insert mode (like vim)
                if cursor.col > 0 {
                    cursor.col -= 1;
                    cursor.desired_col = cursor.col;
                }
                return Some(Mode::Normal);
            }
            KeyCode::Char(c) => {
                buffer.insert_char(cursor.line, cursor.col, c);
                cursor.col += 1;
                cursor.desired_col = cursor.col;
            }
            KeyCode::Backspace => {
                if cursor.col > 0 {
                    cursor.col -= 1;
                    buffer.delete_char(cursor.line, cursor.col);
                    cursor.desired_col = cursor.col;
                } else if cursor.line > 0 {
                    // Join with previous line
                    if let Some(prev_line) = buffer.get_line(cursor.line - 1) {
                        let prev_len = prev_line.len();
                        buffer.join_lines(cursor.line - 1);
                        cursor.line -= 1;
                        cursor.col = prev_len;
                        cursor.desired_col = cursor.col;
                    }
                }
            }
            KeyCode::Enter => {
                buffer.insert_newline(cursor.line, cursor.col);
                cursor.line += 1;
                cursor.col = 0;
                cursor.desired_col = 0;
            }
            KeyCode::Left => cursor.move_left(buffer),
            KeyCode::Right => cursor.move_right(buffer),
            KeyCode::Up => cursor.move_up(buffer),
            KeyCode::Down => cursor.move_down(buffer),
            _ => {}
        }
        None
    }
}

impl Default for InsertMode {
    fn default() -> Self {
        Self::new()
    }
}

