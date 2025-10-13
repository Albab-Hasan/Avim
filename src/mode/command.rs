use crate::buffer::Buffer;
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub struct CommandMode {
    input: String,
}

impl CommandMode {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<CommandResult> {
        match key.code {
            KeyCode::Esc => {
                return Some(CommandResult::Cancel);
            }
            KeyCode::Enter => {
                return Some(CommandResult::Execute(self.input.clone()));
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.pop();
                }
            }
            _ => {}
        }
        None
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }
}

impl Default for CommandMode {
    fn default() -> Self {
        Self::new()
    }
}

pub enum CommandResult {
    Execute(String),
    Cancel,
}

pub fn execute_command(cmd: &str, buffer: &mut Buffer) -> io::Result<CommandAction> {
    let cmd = cmd.trim();
    
    if cmd.is_empty() {
        return Ok(CommandAction::None);
    }

    match cmd {
        "q" | "quit" => Ok(CommandAction::Quit),
        "q!" | "quit!" => Ok(CommandAction::ForceQuit),
        "w" | "write" => {
            buffer.save()?;
            Ok(CommandAction::None)
        }
        "wq" | "x" => {
            buffer.save()?;
            Ok(CommandAction::Quit)
        }
        _ if cmd.starts_with("w ") => {
            let path = cmd[2..].trim();
            buffer.save_as(path)?;
            Ok(CommandAction::None)
        }
        _ if cmd.starts_with("e ") => {
            let path = cmd[2..].trim();
            Ok(CommandAction::Edit(path.to_string()))
        }
        _ => Ok(CommandAction::Error(format!("Unknown command: {}", cmd))),
    }
}

pub enum CommandAction {
    None,
    Quit,
    ForceQuit,
    Edit(String),
    Error(String),
}

