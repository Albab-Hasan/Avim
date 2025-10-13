mod state;

use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::{Mode, NormalMode, InsertMode, VisualMode, CommandMode};
use crate::ui::Renderer;
use crate::command::{execute_command, CommandAction, CommandResult};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

pub use state::EditorState;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    mode: Mode,
    normal_mode: NormalMode,
    insert_mode: InsertMode,
    visual_mode: Option<VisualMode>,
    command_mode: CommandMode,
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
            insert_mode: InsertMode::new(),
            visual_mode: None,
            command_mode: CommandMode::new(),
            renderer,
            viewport_offset: 0,
            quit: false,
            message: None,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.quit {
            self.update_viewport();
            self.renderer.render(
                &self.buffer,
                &self.cursor,
                &self.mode,
                self.viewport_offset,
                &self.command_mode,
                self.visual_mode.as_ref(),
                self.message.as_deref(),
            )?;

            if let Event::Key(key) = event::read()? {
                self.message = None;

                // Handle Ctrl+C for quit in any mode
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.quit = true;
                    continue;
                }

                match self.mode {
                    Mode::Normal => {
                        if let Some(new_mode) = self.normal_mode.handle_key(key, &mut self.cursor, &mut self.buffer) {
                            self.mode = new_mode;
                            if let Mode::Visual(vtype) = new_mode {
                                self.visual_mode = Some(VisualMode::new(vtype, &self.cursor));
                            }
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

