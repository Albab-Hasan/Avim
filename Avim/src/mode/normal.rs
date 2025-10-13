use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::Mode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct NormalMode {
    pending_operator: Option<char>,
    yank_register: Vec<String>,
    operator: Option<Operator>,
}

pub enum NormalAction {
    None,
    ModeChange(Mode),
    StartSearch(bool), // true for forward, false for backward
    NextMatch,
    PrevMatch,
}

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    Delete,
    Change,
    Yank,
}

impl NormalMode {
    pub fn new() -> Self {
        Self {
            pending_operator: None,
            yank_register: Vec::new(),
            operator: None,
        }
    }

    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        cursor: &mut Cursor,
        buffer: &mut Buffer,
    ) -> NormalAction {
        // Handle Ctrl+r for redo
        if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
            if let Some((line, col)) = buffer.redo() {
                cursor.line = line;
                cursor.col = col;
                cursor.desired_col = col;
            }
            return NormalAction::None;
        }

        match key.code {
            KeyCode::Char('h') => cursor.move_left(buffer),
            KeyCode::Char('j') => cursor.move_down(buffer),
            KeyCode::Char('k') => cursor.move_up(buffer),
            KeyCode::Char('l') => cursor.move_right(buffer),
            KeyCode::Char('i') => return NormalAction::ModeChange(Mode::Insert),
            KeyCode::Char('I') => {
                cursor.move_line_start();
                return NormalAction::ModeChange(Mode::Insert);
            }
            KeyCode::Char('a') => {
                cursor.move_right(buffer);
                return NormalAction::ModeChange(Mode::Insert);
            }
            KeyCode::Char('A') => {
                cursor.move_line_end(buffer);
                return NormalAction::ModeChange(Mode::Insert);
            }
            KeyCode::Char('o') => {
                cursor.move_line_end(buffer);
                buffer.insert_newline(cursor.line, buffer.get_line(cursor.line).map(|l| l.len()).unwrap_or(0));
                cursor.line += 1;
                cursor.col = 0;
                cursor.desired_col = 0;
                return NormalAction::ModeChange(Mode::Insert);
            }
            KeyCode::Char('O') => {
                cursor.move_line_start();
                buffer.insert_newline(cursor.line, 0);
                cursor.col = 0;
                cursor.desired_col = 0;
                return NormalAction::ModeChange(Mode::Insert);
            }
            KeyCode::Char('w') => {
                if let Some(op) = self.operator {
                    // Operator with motion (dw, cw, yw)
                    let start_col = cursor.col;
                    cursor.move_word_forward(buffer);
                    let end_col = cursor.col;
                    
                    if let Some(line) = buffer.get_line_mut(cursor.line) {
                        if start_col < line.len() {
                            let deleted: String = line.drain(start_col..end_col.min(line.len())).collect();
                            if op == Operator::Yank || op == Operator::Change {
                                self.yank_register = vec![deleted];
                            }
                        }
                    }
                    cursor.col = start_col;
                    cursor.desired_col = start_col;
                    self.operator = None;
                    
                    if op == Operator::Change {
                        return NormalAction::ModeChange(Mode::Insert);
                    }
                } else {
                    cursor.move_word_forward(buffer);
                }
            }
            KeyCode::Char('b') => cursor.move_word_backward(buffer),
            KeyCode::Char('0') => cursor.move_line_start(),
            KeyCode::Char('$') => {
                if let Some(op) = self.operator {
                    // Operator to end of line (d$, c$, y$)
                    let start_col = cursor.col;
                    if let Some(line) = buffer.get_line_mut(cursor.line) {
                        if start_col < line.len() {
                            let deleted: String = line.drain(start_col..).collect();
                            if op == Operator::Yank || op == Operator::Change {
                                self.yank_register = vec![deleted];
                            }
                        }
                    }
                    self.operator = None;
                    
                    if op == Operator::Change {
                        return NormalAction::ModeChange(Mode::Insert);
                    }
                } else {
                    cursor.move_line_end(buffer);
                }
            }
            KeyCode::Char('g') => {
                // Handle gg
                if self.pending_operator == Some('g') {
                    cursor.move_to_line(0, buffer);
                    self.pending_operator = None;
                } else {
                    self.pending_operator = Some('g');
                }
            }
            KeyCode::Char('G') => {
                cursor.move_to_line(buffer.line_count().saturating_sub(1), buffer);
            }
            KeyCode::Char('x') => {
                buffer.delete_char(cursor.line, cursor.col);
            }
            KeyCode::Char('J') => {
                // Join lines
                buffer.join_lines(cursor.line);
            }
            KeyCode::Char('c') => {
                if self.pending_operator == Some('c') {
                    // cc - change line
                    if let Some(_line) = buffer.delete_line(cursor.line) {
                        buffer.insert_newline(cursor.line.saturating_sub(1), 
                            buffer.get_line(cursor.line.saturating_sub(1)).map(|l| l.len()).unwrap_or(0));
                        cursor.col = 0;
                        cursor.desired_col = 0;
                    }
                    self.pending_operator = None;
                    return NormalAction::ModeChange(Mode::Insert);
                } else if self.operator.is_some() {
                    // Already have an operator, ignore
                    self.operator = None;
                } else {
                    self.operator = Some(Operator::Change);
                    self.pending_operator = Some('c');
                }
            }
            KeyCode::Char('d') => {
                if self.pending_operator == Some('d') {
                    // dd - delete line
                    if let Some(line) = buffer.delete_line(cursor.line) {
                        self.yank_register = vec![line];
                    }
                    self.pending_operator = None;
                    self.operator = None;
                } else if self.operator.is_some() {
                    // Already have an operator, ignore
                    self.operator = None;
                } else {
                    self.operator = Some(Operator::Delete);
                    self.pending_operator = Some('d');
                }
            }
            KeyCode::Char('y') => {
                if self.pending_operator == Some('y') {
                    // yy - yank line
                    if let Some(line) = buffer.get_line(cursor.line) {
                        self.yank_register = vec![line.clone()];
                    }
                    self.pending_operator = None;
                    self.operator = None;
                } else if self.operator.is_some() {
                    // Already have an operator, ignore
                    self.operator = None;
                } else {
                    self.operator = Some(Operator::Yank);
                    self.pending_operator = Some('y');
                }
            }
            KeyCode::Char('p') => {
                // Paste below
                if !self.yank_register.is_empty() {
                    for (i, line) in self.yank_register.iter().enumerate() {
                        buffer.insert_newline(cursor.line + i, buffer.get_line(cursor.line + i).map(|l| l.len()).unwrap_or(0));
                        if let Some(target) = buffer.get_line_mut(cursor.line + i + 1) {
                            *target = line.clone();
                        }
                    }
                    cursor.line += 1;
                }
            }
            KeyCode::Char('P') => {
                // Paste above
                if !self.yank_register.is_empty() {
                    for (i, line) in self.yank_register.iter().enumerate() {
                        buffer.insert_newline(cursor.line + i, 0);
                        if let Some(target) = buffer.get_line_mut(cursor.line + i) {
                            *target = line.clone();
                        }
                    }
                }
            }
            KeyCode::Char('v') => {
                return NormalAction::ModeChange(Mode::Visual(crate::mode::VisualType::Character));
            }
            KeyCode::Char('V') => {
                return NormalAction::ModeChange(Mode::Visual(crate::mode::VisualType::Line));
            }
            KeyCode::Char('u') => {
                // Undo
                if let Some((line, col)) = buffer.undo() {
                    cursor.line = line;
                    cursor.col = col;
                    cursor.desired_col = col;
                }
            }
            KeyCode::Char('/') => {
                return NormalAction::StartSearch(true);
            }
            KeyCode::Char('?') => {
                return NormalAction::StartSearch(false);
            }
            KeyCode::Char('n') => {
                return NormalAction::NextMatch;
            }
            KeyCode::Char('N') => {
                return NormalAction::PrevMatch;
            }
            KeyCode::Char(':') => {
                return NormalAction::ModeChange(Mode::Command);
            }
            _ => {
                self.pending_operator = None;
                self.operator = None;
            }
        }
        NormalAction::None
    }

    pub fn yank_register(&self) -> &[String] {
        &self.yank_register
    }
}

impl Default for NormalMode {
    fn default() -> Self {
        Self::new()
    }
}

