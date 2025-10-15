mod state;

use crate::buffer::Buffer;
use crate::mode::{Mode, NormalMode, InsertMode, VisualMode, CommandMode};
use crate::ui::Renderer;
use crate::command::{execute_command, CommandAction, CommandResult};
use crate::search::SearchState;
use crate::window::WindowManager;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

pub use state::EditorState;

pub struct Editor {
    window_manager: WindowManager,
    mode: Mode,
    normal_mode: NormalMode,
    visual_mode: Option<VisualMode>,
    command_mode: CommandMode,
    search_state: SearchState,
    search_input: String,
    in_search: bool,
    renderer: Renderer,
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

        let window_manager = WindowManager::new(buffer);

        Ok(Self {
            window_manager,
            mode: Mode::Normal,
            normal_mode: NormalMode::new(),
            visual_mode: None,
            command_mode: CommandMode::new(),
            search_state: SearchState::new(),
            search_input: String::new(),
            in_search: false,
            renderer,
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
                &self.window_manager,
                &self.mode,
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
                            self.search_state.search(self.window_manager.get_active_buffer(), &self.search_input, forward);
                            if let Some((line, col)) = self.search_state.current() {
                                let mut cursor = self.window_manager.get_active_cursor();
                                cursor.line = line;
                                cursor.col = col;
                                cursor.desired_col = col;
                                self.window_manager.set_active_cursor(cursor);
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
                        let mut cursor = self.window_manager.get_active_cursor();
                        match self.normal_mode.handle_key(key, &mut cursor, self.window_manager.get_active_buffer_mut()) {
                            NormalAction::ModeChange(new_mode) => {
                                self.mode = new_mode;
                                if let Mode::Visual(vtype) = new_mode {
                                    self.visual_mode = Some(VisualMode::new(vtype, &cursor));
                                }
                            }
                            NormalAction::StartSearch(forward) => {
                                self.in_search = true;
                                self.search_input.clear();
                                self.search_state.forward = forward;
                            }
                            NormalAction::NextMatch => {
                                if let Some((line, col)) = self.search_state.next_match() {
                                    cursor.line = line;
                                    cursor.col = col;
                                    cursor.desired_col = col;
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
                                    cursor.line = line;
                                    cursor.col = col;
                                    cursor.desired_col = col;
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
                            NormalAction::WindowCommand => {
                                // Read next key for window command
                                if let Event::Key(next_key) = event::read()? {
                                    if let Some(cmd) = self.normal_mode.handle_window_command(next_key) {
                                        match cmd.as_str() {
                                            "next_window" => self.window_manager.next_window(),
                                            "prev_window" => self.window_manager.prev_window(),
                                            "split_horizontal" => {
                                                if let Err(e) = self.window_manager.split_horizontal(None) {
                                                    self.message = Some(e);
                                                }
                                            }
                                            "split_vertical" => {
                                                if let Err(e) = self.window_manager.split_vertical(None) {
                                                    self.message = Some(e);
                                                }
                                            }
                                            "close_window" => {
                                                if let Err(e) = self.window_manager.close_window() {
                                                    self.message = Some(e);
                                                }
                                            }
                                            "close_other_windows" => {
                                                self.message = Some("Close other windows not yet implemented".to_string());
                                            }
                                            "increase_height" | "decrease_height" | "increase_width" | "decrease_width" | "equal_size" => {
                                                self.message = Some("Window resizing not yet implemented".to_string());
                                            }
                                            _ if cmd.starts_with("navigate_") => {
                                                let direction = cmd.chars().last().unwrap_or('h');
                                                self.window_manager.navigate_to_window(direction);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            NormalAction::None => {}
                        }
                        self.window_manager.set_active_cursor(cursor);
                    }
                    Mode::Insert => {
                        let mut cursor = self.window_manager.get_active_cursor();
                        if let Some(new_mode) = InsertMode::handle_key(key, &mut cursor, self.window_manager.get_active_buffer_mut()) {
                            self.mode = new_mode;
                        }
                        self.window_manager.set_active_cursor(cursor);
                    }
                    Mode::Visual(_) => {
                        if let Some(ref mut visual) = self.visual_mode {
                            let mut cursor = self.window_manager.get_active_cursor();
                            if let Some(new_mode) = visual.handle_key(key, &mut cursor, self.window_manager.get_active_buffer_mut()) {
                                self.mode = new_mode;
                                self.visual_mode = None;
                            }
                            self.window_manager.set_active_cursor(cursor);
                        }
                    }
                    Mode::Command => {
                        if let Some(result) = self.command_mode.handle_key(key) {
                            match result {
                                CommandResult::Execute(cmd) => {
                                    match execute_command(&cmd, self.window_manager.get_active_buffer_mut()) {
                                        Ok(action) => {
                                            match action {
                                                CommandAction::Quit => {
                                                    if self.window_manager.get_active_buffer().is_modified() {
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
                                                            // Replace current buffer with new one
                                                            let buffer_count = self.window_manager.get_buffers().len();
                                                            self.window_manager.get_buffers_mut().push(new_buffer);
                                                            let current_window = self.window_manager.get_active_window_mut();
                                                            current_window.buffer_id = buffer_count;
                                                            current_window.cursor_line = 0;
                                                            current_window.cursor_col = 0;
                                                            current_window.viewport_offset = 0;
                                                        }
                                                        Err(e) => {
                                                            self.message = Some(format!("Error: {}", e));
                                                        }
                                                    }
                                                }
                                                CommandAction::SplitHorizontal(file_path) => {
                                                    if let Err(e) = self.window_manager.split_horizontal(file_path.as_deref()) {
                                                        self.message = Some(e);
                                                    }
                                                }
                                                CommandAction::SplitVertical(file_path) => {
                                                    if let Err(e) = self.window_manager.split_vertical(file_path.as_deref()) {
                                                        self.message = Some(e);
                                                    }
                                                }
                                                CommandAction::CloseWindow => {
                                                    if let Err(e) = self.window_manager.close_window() {
                                                        self.message = Some(e);
                                                    }
                                                }
                                                CommandAction::CloseOtherWindows => {
                                                    // TODO: Implement close other windows
                                                    self.message = Some("Close other windows not yet implemented".to_string());
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
        let cursor = self.window_manager.get_active_cursor();
        let viewport_offset = self.window_manager.get_viewport_offset();
        
        if cursor.line < viewport_offset {
            self.window_manager.set_viewport_offset(cursor.line);
        } else if cursor.line >= viewport_offset + terminal_height {
            self.window_manager.set_viewport_offset(cursor.line - terminal_height + 1);
        }
    }
}

