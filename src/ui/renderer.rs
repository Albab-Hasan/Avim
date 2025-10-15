use crate::mode::{Mode, VisualMode};
use crate::command::CommandMode;
use crate::ui::StatusLine;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write, Stdout};
use syntect::highlighting::Color as SyntectColor;

pub struct Renderer {
    stdout: Stdout,
    width: u16,
    height: u16,
    last_cursor: (usize, usize),
    needs_full_redraw: bool,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            width,
            height,
            last_cursor: (0, 0),
            needs_full_redraw: true,
        })
    }

    pub fn enter(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn render(
        &mut self,
        window_manager: &crate::window::WindowManager,
        mode: &Mode,
        command_mode: &CommandMode,
        visual_mode: Option<&VisualMode>,
        message: Option<&str>,
    ) -> io::Result<()> {
        // Update terminal size
        let (width, height) = terminal::size()?;
        if self.width != width || self.height != height {
        self.width = width;
        self.height = height;
            self.needs_full_redraw = true;
        }

        // Always clear screen to ensure clean rendering
        execute!(self.stdout, terminal::Clear(ClearType::All))?;

        // Get active window info
        let _active_window = window_manager.get_active_window();
        let active_buffer = window_manager.get_active_buffer();
        let active_cursor = window_manager.get_active_cursor();
        let viewport_offset = window_manager.get_viewport_offset();

        // Check if we have multiple windows and show indicator
        let window_count = window_manager.get_window_count();
        if window_count > 1 {
            // Show window indicator at top
            execute!(
                self.stdout,
                cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Yellow),
                Print(format!("Windows: {} (Active: {}) - Use Ctrl+w to navigate", 
                    window_count, 
                    window_manager.get_active_window().buffer_id + 1
                )),
                ResetColor
            )?;
        }

        let visible_lines = if window_count > 1 {
            (height as usize).saturating_sub(3) // Account for window indicator
        } else {
            (height as usize).saturating_sub(2) // Normal mode
        };
        let line_num_width = (active_buffer.line_count().to_string().len() + 1) as u16;
        
        // Build entire screen output in memory first
        let mut screen_buffer = String::with_capacity(visible_lines * 100);
        
        for row in 0..visible_lines {
            let line_idx = viewport_offset + row;
            
            if line_idx < active_buffer.line_count() {
                // Line number
                screen_buffer.push_str(&format!("\x1b[33m{:>width$} \x1b[0m", 
                    line_idx + 1, 
                    width = line_num_width as usize - 1
                ));
                
                if let Some(line) = active_buffer.get_line(line_idx) {
                    // Check if this line is in visual selection
                    if let Some(visual) = visual_mode {
                        if let Mode::Visual(_) = mode {
                            let (start_line, start_col, end_line, end_col) = visual.get_selection(&active_cursor);
                            
                            if line_idx >= start_line && line_idx <= end_line {
                                // Highlight selected portion with syntax highlighting
                                let highlighted = active_buffer.highlight_line(line_idx);
                                let _chars: Vec<char> = line.chars().collect();
                                if start_line == end_line {
                                    let mut char_idx = 0;
                                    for (style, text) in highlighted {
                                        for c in text.chars() {
                                            if char_idx >= start_col && char_idx <= end_col {
                                                screen_buffer.push_str("\x1b[48;5;240m\x1b[37m");
                                                screen_buffer.push(c);
                                                screen_buffer.push_str("\x1b[0m");
                                            } else {
                                                screen_buffer.push_str(&Self::rgb_to_ansi(style.foreground));
                                                screen_buffer.push(c);
                                                screen_buffer.push_str("\x1b[0m");
                                            }
                                            char_idx += 1;
                                        }
                                    }
                                } else {
                                    // Multi-line selection - just use syntax highlighting
                                    for (style, text) in highlighted {
                                        screen_buffer.push_str(&Self::rgb_to_ansi(style.foreground));
                                        screen_buffer.push_str(&text);
                                        screen_buffer.push_str("\x1b[0m");
                                    }
                                }
                                screen_buffer.push_str("\r\n");
                                continue;
                            }
                        }
                    }
                    
                    // Apply syntax highlighting to the line
                    let highlighted = active_buffer.highlight_line(line_idx);
                    if highlighted.is_empty() {
                        // Fallback for empty lines or no highlighting
                        screen_buffer.push_str(line);
                    } else {
                        // Ensure we're using the current line content
                        let current_line = active_buffer.get_line(line_idx).map_or("", |v| v);
                        let mut char_pos = 0;
                        let mut bracket_style = None;
                        
                        for (style, text) in highlighted {
                            // Make sure we don't exceed the current line length
                            if char_pos < current_line.len() {
                                let end_pos = (char_pos + text.len()).min(current_line.len());
                                let actual_text = &current_line[char_pos..end_pos];
                                
                                // Check if this segment contains brackets and normalize their color
                                let normalized_style = if Self::contains_brackets(actual_text) {
                                    // Use a consistent color for brackets
                                    if bracket_style.is_none() {
                                        bracket_style = Some(style);
                                    }
                                    bracket_style.unwrap_or(style)
                                } else {
                                    style
                                };
                                
                                screen_buffer.push_str(&Self::rgb_to_ansi(normalized_style.foreground));
                                screen_buffer.push_str(actual_text);
                                screen_buffer.push_str("\x1b[0m");
                                char_pos = end_pos;
                            }
                        }
                        // Add any remaining characters that weren't highlighted
                        if char_pos < current_line.len() {
                            screen_buffer.push_str(&current_line[char_pos..]);
                        }
                    }
                }
            } else {
                screen_buffer.push_str(&format!("\x1b[34m{:>width$} ~\x1b[0m", 
                    "", 
                    width = line_num_width as usize - 1
                ));
            }
            
            if row < visible_lines - 1 {
                screen_buffer.push_str("\r\n");
            }
        }
        
        // Write entire screen in one go
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            Print(&screen_buffer)
        )?;

        // Render status line
        let status_line = StatusLine::new(mode, active_buffer, &active_cursor);
        let status_row = if window_count > 1 {
            visible_lines + 1 // Account for window indicator
        } else {
            visible_lines
        };
        self.render_status_line(&status_line, status_row as u16)?;

        // Render command line or message
        let cmd_line_row = if window_count > 1 {
            visible_lines + 2 // Account for window indicator
        } else {
            visible_lines + 1
        };
        execute!(self.stdout, cursor::MoveTo(0, cmd_line_row as u16))?;
        
        // Clear the command line area
        execute!(
            self.stdout,
            terminal::Clear(ClearType::CurrentLine)
        )?;
        
        if let Mode::Command = mode {
            execute!(
                self.stdout,
                SetForegroundColor(Color::Yellow),
                Print(":"),
                Print(command_mode.input()),
                ResetColor
            )?;
        } else if let Some(msg) = message {
            execute!(self.stdout, Print(msg))?;
        }

        // Position cursor
        if let Mode::Command = mode {
            // In command mode, position cursor at end of command input
            let cmd_col = 1 + command_mode.input().len(); // 1 for the ':'
            let cmd_row = if window_count > 1 {
                visible_lines + 2 // Account for window indicator
            } else {
                visible_lines + 1
            };
            execute!(
                self.stdout,
                cursor::MoveTo(cmd_col as u16, cmd_row as u16),
                cursor::Show
            )?;
                  } else {
                      // Normal cursor positioning in text area
                      let screen_row = active_cursor.line.saturating_sub(viewport_offset);
                      let line_num_width = (active_buffer.line_count().to_string().len() + 1) as u16;
                      let screen_col = (active_cursor.col + line_num_width as usize).min((width as usize).saturating_sub(1));
                      
                      // Ensure cursor is visible on screen
                      if screen_row >= visible_lines {
                          return Ok(());
                      }
                      
                      // Adjust for window indicator if multiple windows
                      let adjusted_row = if window_count > 1 {
                          screen_row + 1
                      } else {
                          screen_row
                      };
                      
                      // Update cursor position
                      self.last_cursor = (active_cursor.line, active_cursor.col);
        
        execute!(
            self.stdout,
            cursor::MoveTo(screen_col as u16, adjusted_row as u16),
            cursor::Show
        )?;
                  }

        self.stdout.flush()?;
        Ok(())
    }

    pub fn force_redraw(&mut self) {
        self.needs_full_redraw = true;
    }
    
    fn rgb_to_ansi(color: SyntectColor) -> String {
        format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b)
    }
    
    fn contains_brackets(text: &str) -> bool {
        text.chars().any(|c| matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'' | '`'))
    }
    

    #[allow(dead_code)]
    fn render_line_with_highlight(&mut self, line: &str, start: usize, end: usize) -> io::Result<()> {
        let chars: Vec<char> = line.chars().collect();
        
        // Before highlight
        for i in 0..start.min(chars.len()) {
            execute!(self.stdout, Print(chars[i]))?;
        }
        
        // Highlighted portion
        execute!(
            self.stdout,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;
        
        for i in start..end.min(chars.len()) {
            execute!(self.stdout, Print(chars[i]))?;
        }
        
        execute!(self.stdout, ResetColor)?;
        
        // After highlight
        for i in end..chars.len() {
            execute!(self.stdout, Print(chars[i]))?;
        }
        
        Ok(())
    }

    fn render_status_line(&mut self, status_line: &StatusLine, row: u16) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, row),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;

        let status_text = status_line.render(self.width as usize);
        execute!(self.stdout, Print(status_text))?;

        execute!(self.stdout, ResetColor)?;
        Ok(())
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }
}

