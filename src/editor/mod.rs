mod state;

use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::{Mode, NormalMode, InsertMode, VisualMode, CommandMode};
use crate::ui::Renderer;
use crate::command::{execute_command, CommandAction, CommandResult};
use crate::search::SearchState;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

pub use state::EditorState;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    mode: Mode,
    normal_mode: NormalMode,
    visual_mode: Option<VisualMode>,
    command_mode: CommandMode,
    search_state: SearchState,
    search_input: String,
    in_search: bool,
    renderer: Renderer,
    viewport_offset: usize,
    quit: bool,
    message: Option<String>,
}

impl Editor {
    pub fn new(file_path: Option<&str>) -> io::Result<Self> {
        let buffer = if let Some(path) = file_path {
            Buffer::from_file(path)?
        } else {
            Buffer::new()
        };

        let mut renderer = Renderer::new()?;
        renderer.enter()?;

        Ok(Self {
            buffer,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            normal_mode: NormalMode::new(),
            visual_mode: None,
            command_mode: CommandMode::new(),
            search_state: SearchState::new(),
            search_input: String::new(),
            in_search: false,
            renderer,
            viewport_offset: 0,
            quit: false,
            message: None,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.quit {
            self.update_viewport();
            
            let status_message = if self.in_search {
                let prefix = if self.search_state.forward { "/" } else { "?" };
                Some(format!("{}{}", prefix, self.search_input))
            } else {
                self.message.as_deref().map(|s| s.to_string())
            };

            self.renderer.render(
                &self.buffer,
                &self.cursor,
                &self.mode,
                self.viewport_offset,
                &self.command_mode,
                self.visual_mode.as_ref(),
                status_message.as_deref(),
            )?;

            if let Event::Key(key) = event::read()? {
                // Filter out key release events to prevent double input
                if matches!(key.kind, crossterm::event::KeyEventKind::Release) {
                    continue;
                }
                
                self.message = None;

                // Handle Ctrl+C for quit in any mode
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.quit = true;
                    continue;
                }

                // Handle search input
                if self.in_search {
                    match key.code {
                        KeyCode::Esc => {
                            self.in_search = false;
                            self.search_input.clear();
                        }
                        KeyCode::Enter => {
                            let forward = self.search_state.forward;
                            self.search_state.search(&self.buffer, &self.search_input, forward);
                            if let Some((line, col)) = self.search_state.current() {
                                self.cursor.line = line;
                                self.cursor.col = col;
                                self.cursor.desired_col = col;
                                self.message = Some(format!(
                                    "Match 1 of {} for '{}'",
                                    self.search_state.match_count(),
                                    self.search_input
                                ));
                            } else {
                                self.message = Some(format!("Pattern not found: {}", self.search_input));
                            }
                            self.in_search = false;
                            self.search_input.clear();
                        }
                        KeyCode::Backspace => {
                            self.search_input.pop();
                        }
                        KeyCode::Char(c) => {
                            self.search_input.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                match self.mode {
                    Mode::Normal => {
                        use crate::mode::NormalAction;
                        match self.normal_mode.handle_key(key, &mut self.cursor, &mut self.buffer) {
                            NormalAction::ModeChange(new_mode) => {
                                self.mode = new_mode;
                                if let Mode::Visual(vtype) = new_mode {
                                    self.visual_mode = Some(VisualMode::new(vtype, &self.cursor));
                                }
                            }
                            NormalAction::StartSearch(forward) => {
                                self.in_search = true;
                                self.search_input.clear();
                                self.search_state.forward = forward;
                            }
                            NormalAction::NextMatch => {
                                if let Some((line, col)) = self.search_state.next_match() {
                                    self.cursor.line = line;
                                    self.cursor.col = col;
                                    self.cursor.desired_col = col;
                                    if let Some(current) = self.search_state.current_match {
                                        self.message = Some(format!(
                                            "Match {} of {}",
                                            current + 1,
                                            self.search_state.match_count()
                                        ));
                                    }
                                } else {
                                    self.message = Some("No search pattern".to_string());
                                }
                            }
                            NormalAction::PrevMatch => {
                                if let Some((line, col)) = self.search_state.prev_match() {
                                    self.cursor.line = line;
                                    self.cursor.col = col;
                                    self.cursor.desired_col = col;
                                    if let Some(current) = self.search_state.current_match {
                                        self.message = Some(format!(
                                            "Match {} of {}",
                                            current + 1,
                                            self.search_state.match_count()
                                        ));
                                    }
                                } else {
                                    self.message = Some("No search pattern".to_string());
                                }
                            }
                            NormalAction::None => {}
                        }
                    }
                    Mode::Insert => {
                        if let Some(new_mode) = InsertMode::handle_key(key, &mut self.cursor, &mut self.buffer) {
                            self.mode = new_mode;
                        }
                    }
                    Mode::Visual(_) => {
                        if let Some(ref mut visual) = self.visual_mode {
                            if let Some(new_mode) = visual.handle_key(key, &mut self.cursor, &mut self.buffer) {
                                self.mode = new_mode;
                                self.visual_mode = None;
                            }
                        }
                    }
                    Mode::Command => {
                        if let Some(result) = self.command_mode.handle_key(key) {
                            match result {
                                CommandResult::Execute(cmd) => {
                                    match execute_command(&cmd, &mut self.buffer) {
                                        Ok(action) => {
                                            match action {
                                                CommandAction::Quit => {
                                                    if self.buffer.is_modified() {
                                                        self.message = Some("No write since last change (use :q! to override)".to_string());
                                                    } else {
                                                        self.quit = true;
                                                    }
                                                }
                                                CommandAction::ForceQuit => {
                                                    self.quit = true;
                                                }
                                                CommandAction::Edit(path) => {
                                                    match Buffer::from_file(&path) {
                                                        Ok(new_buffer) => {
                                                            self.buffer = new_buffer;
                                                            self.cursor = Cursor::new();
                                                        }
                                                        Err(e) => {
                                                            self.message = Some(format!("Error: {}", e));
                                                        }
                                                    }
                                                }
                                                CommandAction::Error(msg) => {
                                                    self.message = Some(msg);
                                                }
                                                CommandAction::None => {}
                                            }
                                        }
                                        Err(e) => {
                                            self.message = Some(format!("Error: {}", e));
                                        }
                                    }
                                    self.command_mode.clear();
                                    self.mode = Mode::Normal;
                                }
                                CommandResult::Cancel => {
                                    self.command_mode.clear();
                                    self.mode = Mode::Normal;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.renderer.exit()?;
        Ok(())
    }

    fn update_viewport(&mut self) {
        let terminal_height = self.renderer.height().saturating_sub(2); // Leave room for status line
        
        if self.cursor.line < self.viewport_offset {
            self.viewport_offset = self.cursor.line;
        } else if self.cursor.line >= self.viewport_offset + terminal_height {
            self.viewport_offset = self.cursor.line - terminal_height + 1;
        }
    }
}

