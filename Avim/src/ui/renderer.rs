use crate::buffer::Buffer;
use crate::cursor::Cursor;
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

pub struct Renderer {
    stdout: Stdout,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            width,
            height,
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
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &Mode,
        viewport_offset: usize,
        command_mode: &CommandMode,
        visual_mode: Option<&VisualMode>,
        message: Option<&str>,
    ) -> io::Result<()> {
        // Update terminal size
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;

        execute!(self.stdout, terminal::Clear(ClearType::All))?;

        // Render buffer lines
        let visible_lines = (height as usize).saturating_sub(2);
        let line_num_width = (buffer.line_count().to_string().len() + 1) as u16;
        
        for row in 0..visible_lines {
            let line_idx = viewport_offset + row;
            
            execute!(self.stdout, cursor::MoveTo(0, row as u16))?;

            if line_idx < buffer.line_count() {
                // Render line number
                execute!(
                    self.stdout,
                    SetForegroundColor(Color::DarkYellow),
                    Print(format!("{:>width$} ", line_idx + 1, width = line_num_width as usize - 1)),
                    ResetColor
                )?;
                
                if let Some(line) = buffer.get_line(line_idx) {
                    // Check if this line is in visual selection
                    if let Some(visual) = visual_mode {
                        if let Mode::Visual(_) = mode {
                            let (start_line, start_col, end_line, end_col) = visual.get_selection(cursor);
                            
                            if line_idx >= start_line && line_idx <= end_line {
                                // Highlight selected portion
                                if start_line == end_line {
                                    // Single line selection
                                    self.render_line_with_highlight(line, start_col, end_col)?;
                                    continue;
                                } else if line_idx == start_line {
                                    self.render_line_with_highlight(line, start_col, line.len())?;
                                    continue;
                                } else if line_idx == end_line {
                                    self.render_line_with_highlight(line, 0, end_col)?;
                                    continue;
                                } else {
                                    self.render_line_with_highlight(line, 0, line.len())?;
                                    continue;
                                }
                            }
                        }
                    }
                    
                    execute!(self.stdout, Print(line))?;
                }
            } else {
                execute!(
                    self.stdout,
                    SetForegroundColor(Color::DarkBlue),
                    Print(format!("{:>width$} ~", "", width = line_num_width as usize - 1)),
                    ResetColor
                )?;
            }
        }

        // Render status line
        let status_line = StatusLine::new(mode, buffer, cursor);
        self.render_status_line(&status_line, visible_lines as u16)?;

        // Render command line or message
        execute!(self.stdout, cursor::MoveTo(0, (visible_lines + 1) as u16))?;
        
        if let Mode::Command = mode {
            execute!(
                self.stdout,
                Print(":"),
                Print(command_mode.input())
            )?;
        } else if let Some(msg) = message {
            execute!(self.stdout, Print(msg))?;
        }

        // Position cursor (accounting for line numbers)
        let screen_row = cursor.line.saturating_sub(viewport_offset);
        let line_num_width = (buffer.line_count().to_string().len() + 1) as u16;
        let screen_col = (cursor.col + line_num_width as usize).min((width as usize).saturating_sub(1));
        
        execute!(
            self.stdout,
            cursor::MoveTo(screen_col as u16, screen_row as u16),
            cursor::Show
        )?;

        self.stdout.flush()?;
        Ok(())
    }

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

