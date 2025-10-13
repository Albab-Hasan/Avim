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
                // Handle auto-closing brackets and parentheses
                if let Some(closing_char) = Self::get_closing_char(c) {
                    buffer.insert_char(cursor.line, cursor.col, c);
                    buffer.insert_char(cursor.line, cursor.col + 1, closing_char);
                    cursor.col += 1;
                    cursor.desired_col = cursor.col;
                } else {
                    buffer.insert_char(cursor.line, cursor.col, c);
                    cursor.col += 1;
                    cursor.desired_col = cursor.col;
                }
            }
            KeyCode::Tab => {
                // Insert 4 spaces for indentation
                for _ in 0..4 {
                    buffer.insert_char(cursor.line, cursor.col, ' ');
                    cursor.col += 1;
                }
                cursor.desired_col = cursor.col;
            }
            KeyCode::Backspace => {
                if cursor.col > 0 {
                    let current_line = buffer.get_line(cursor.line).map_or("", |v| v);
                    
                    // Check for smart bracket deletion
                    if let Some(closing_char) = Self::get_closing_char_for_deletion(current_line, cursor.col - 1) {
                        // Check if the next character is the matching closing bracket
                        if cursor.col < current_line.len() && current_line.chars().nth(cursor.col) == Some(closing_char) {
                            // Delete both opening and closing brackets
                            buffer.delete_char(cursor.line, cursor.col - 1);
                            buffer.delete_char(cursor.line, cursor.col - 1);
                            cursor.col -= 1;
                            cursor.desired_col = cursor.col;
                        } else {
                            // Normal backspace
                            buffer.delete_char(cursor.line, cursor.col - 1);
                            cursor.col -= 1;
                            cursor.desired_col = cursor.col;
                        }
                    } else {
                        // Normal backspace
                        buffer.delete_char(cursor.line, cursor.col - 1);
                        cursor.col -= 1;
                        cursor.desired_col = cursor.col;
                    }
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
                // Get current line to determine indentation
                let current_line = buffer.get_line(cursor.line).map_or("", |v| v);
                let base_indent = Self::get_line_indent(current_line);
                
                // Check if we're after an opening bracket/brace for auto-indent
                let extra_indent = if cursor.col > 0 {
                    let char_before = current_line.chars().nth(cursor.col - 1);
                    match char_before {
                        Some('{') | Some('(') | Some('[') => 4, // Add extra indentation
                        _ => 0,
                    }
                } else {
                    0
                };
                
                buffer.insert_newline(cursor.line, cursor.col);
                cursor.line += 1;
                cursor.col = 0;
                cursor.desired_col = 0;
                
                // Add indentation to the new line
                let total_indent = base_indent + extra_indent;
                for _ in 0..total_indent {
                    buffer.insert_char(cursor.line, cursor.col, ' ');
                    cursor.col += 1;
                }
                cursor.desired_col = cursor.col;
            }
            KeyCode::Left => cursor.move_left(buffer),
            KeyCode::Right => cursor.move_right(buffer),
            KeyCode::Up => cursor.move_up(buffer),
            KeyCode::Down => cursor.move_down(buffer),
            _ => {}
        }
        None
    }

    fn get_closing_char(opening: char) -> Option<char> {
        match opening {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '"' => Some('"'),
            '\'' => Some('\''),
            '`' => Some('`'),
            _ => None,
        }
    }
    
    fn get_line_indent(line: &str) -> usize {
        let mut indent = 0;
        for c in line.chars() {
            if c == ' ' {
                indent += 1;
            } else {
                break;
            }
        }
        indent
    }
    
    fn get_closing_char_for_deletion(line: &str, pos: usize) -> Option<char> {
        if let Some(char_at_pos) = line.chars().nth(pos) {
            match char_at_pos {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '"' => Some('"'),
                '\'' => Some('\''),
                '`' => Some('`'),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Default for InsertMode {
    fn default() -> Self {
        Self::new()
    }
}

