use crate::buffer::Buffer;
use crate::cursor::Cursor;
use super::{Window, SplitType, LayoutNode};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WindowBounds {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl WindowBounds {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}

pub struct WindowManager {
    windows: Vec<Window>,
    buffers: Vec<Buffer>,
    active_window: usize,
    layout_root: Option<Box<LayoutNode>>,
    window_bounds: HashMap<usize, WindowBounds>,
}

impl WindowManager {
    pub fn new(initial_buffer: Buffer) -> Self {
        let mut windows = Vec::new();
        let mut buffers = Vec::new();
        let mut window_bounds = HashMap::new();
        
        // Create initial window and buffer
        buffers.push(initial_buffer);
        let window = Window::new(0);
        windows.push(window);
        window_bounds.insert(0, WindowBounds::new(0, 0, 80, 24)); // Default bounds
        
        Self {
            windows,
            buffers,
            active_window: 0,
            layout_root: Some(Box::new(LayoutNode::Leaf { window_id: 0 })),
            window_bounds,
        }
    }

    pub fn get_active_window(&self) -> &Window {
        &self.windows[self.active_window]
    }

    pub fn get_active_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.active_window]
    }

    pub fn get_active_buffer(&self) -> &Buffer {
        &self.buffers[self.windows[self.active_window].buffer_id]
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        let buffer_id = self.windows[self.active_window].buffer_id;
        &mut self.buffers[buffer_id]
    }

    pub fn get_active_cursor(&self) -> Cursor {
        let window = &self.windows[self.active_window];
        Cursor {
            line: window.cursor_line,
            col: window.cursor_col,
            desired_col: window.cursor_col,
        }
    }

    pub fn set_active_cursor(&mut self, cursor: Cursor) {
        let window = &mut self.windows[self.active_window];
        window.cursor_line = cursor.line;
        window.cursor_col = cursor.col;
    }

    pub fn get_viewport_offset(&self) -> usize {
        self.windows[self.active_window].viewport_offset
    }

    pub fn set_viewport_offset(&mut self, offset: usize) {
        self.windows[self.active_window].viewport_offset = offset;
    }

    pub fn split_horizontal(&mut self, file_path: Option<&str>) -> Result<(), String> {
        self.split_window(SplitType::Horizontal, file_path)
    }

    pub fn split_vertical(&mut self, file_path: Option<&str>) -> Result<(), String> {
        self.split_window(SplitType::Vertical, file_path)
    }

    fn split_window(&mut self, split_type: SplitType, file_path: Option<&str>) -> Result<(), String> {
        let current_window_id = self.active_window;
        let current_buffer_id = self.windows[current_window_id].buffer_id;
        
        // Create new buffer if file specified, otherwise use same buffer
        let new_buffer_id = if let Some(path) = file_path {
            match Buffer::from_file(path) {
                Ok(buffer) => {
                    self.buffers.push(buffer);
                    self.buffers.len() - 1
                }
                Err(_) => return Err(format!("Cannot open file: {}", path)),
            }
        } else {
            current_buffer_id
        };

        // Create new window
        let new_window = Window::new(new_buffer_id);
        let new_window_id = self.windows.len();
        self.windows.push(new_window);
        
        // Update layout tree
        self.update_layout_tree(current_window_id, new_window_id, split_type);
        
        // Calculate new window bounds
        self.calculate_window_bounds();
        
        Ok(())
    }

    fn update_layout_tree(&mut self, current_window_id: usize, new_window_id: usize, split_type: SplitType) {
        if let Some(ref mut root) = self.layout_root {
            // Find the current window in the tree and replace it with a split
            Self::replace_window_in_tree(root, current_window_id, new_window_id, split_type);
        }
    }

    fn replace_window_in_tree(node: &mut LayoutNode, window_id: usize, new_window_id: usize, split_type: SplitType) {
        match node {
            LayoutNode::Leaf { window_id: id } if *id == window_id => {
                // Replace this leaf with a split containing both windows
                *node = LayoutNode::Split {
                    split_type,
                    ratio: 0.5,
                    children: vec![
                        Box::new(LayoutNode::Leaf { window_id }),
                        Box::new(LayoutNode::Leaf { window_id: new_window_id }),
                    ],
                };
            }
            LayoutNode::Split { children, .. } => {
                for child in children.iter_mut() {
                    Self::replace_window_in_tree(child, window_id, new_window_id, split_type);
                }
            }
            _ => {}
        }
    }

    fn calculate_window_bounds(&mut self) {
        if let Some(ref root) = self.layout_root {
            Self::calculate_bounds_recursive(&mut self.window_bounds, root, 0, 0, 80, 24); // Default terminal size
        }
    }

    fn calculate_bounds_recursive(window_bounds: &mut HashMap<usize, WindowBounds>, node: &LayoutNode, x: usize, y: usize, width: usize, height: usize) {
        match node {
            LayoutNode::Leaf { window_id } => {
                window_bounds.insert(*window_id, WindowBounds::new(x, y, width, height));
            }
            LayoutNode::Split { split_type, ratio, children } => {
                if children.len() != 2 {
                    return; // Invalid split
                }

                match split_type {
                    SplitType::Horizontal => {
                        let split_height = (height as f32 * ratio) as usize;
                        let remaining_height = height - split_height;
                        
                        Self::calculate_bounds_recursive(window_bounds, &children[0], x, y, width, split_height);
                        Self::calculate_bounds_recursive(window_bounds, &children[1], x, y + split_height, width, remaining_height);
                    }
                    SplitType::Vertical => {
                        let split_width = (width as f32 * ratio) as usize;
                        let remaining_width = width - split_width;
                        
                        Self::calculate_bounds_recursive(window_bounds, &children[0], x, y, split_width, height);
                        Self::calculate_bounds_recursive(window_bounds, &children[1], x + split_width, y, remaining_width, height);
                    }
                }
            }
        }
    }

    pub fn close_window(&mut self) -> Result<(), String> {
        if self.windows.len() <= 1 {
            return Err("Cannot close the last window".to_string());
        }

        let window_to_close = self.active_window;
        
        // Remove window from layout tree
        self.remove_window_from_tree(window_to_close);
        
        // Remove window and its bounds
        self.windows.remove(window_to_close);
        self.window_bounds.remove(&window_to_close);
        
        // Update active window index
        if self.active_window >= self.windows.len() {
            self.active_window = self.windows.len() - 1;
        }
        
        // Recalculate bounds
        self.calculate_window_bounds();
        
        Ok(())
    }

    fn remove_window_from_tree(&mut self, window_id: usize) {
        if let Some(ref mut root) = self.layout_root {
            Self::remove_window_recursive(root, window_id);
        }
    }

    fn remove_window_recursive(node: &mut LayoutNode, window_id: usize) -> bool {
        match node {
            LayoutNode::Leaf { window_id: id } => *id == window_id,
            LayoutNode::Split { children, .. } => {
                for i in 0..children.len() {
                    if Self::remove_window_recursive(&mut children[i], window_id) {
                        children.remove(i);
                        if children.len() == 1 {
                            // Replace split with remaining child
                            *node = *children[0].clone();
                        }
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn next_window(&mut self) {
        self.active_window = (self.active_window + 1) % self.windows.len();
    }

    pub fn prev_window(&mut self) {
        if self.active_window == 0 {
            self.active_window = self.windows.len() - 1;
        } else {
            self.active_window -= 1;
        }
    }

    pub fn navigate_to_window(&mut self, direction: char) -> bool {
        // For now, just cycle through windows
        // TODO: Implement directional navigation based on window positions
        match direction {
            'h' | 'j' | 'k' | 'l' => {
                self.next_window();
                true
            }
            _ => false,
        }
    }

    pub fn get_window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn get_window_bounds(&self, window_id: usize) -> Option<&WindowBounds> {
        self.window_bounds.get(&window_id)
    }

    pub fn get_layout_root(&self) -> Option<&LayoutNode> {
        self.layout_root.as_ref().map(|r| r.as_ref())
    }

    pub fn resize_window(&mut self, _delta: f32) {
        // TODO: Implement window resizing by adjusting split ratios
        // This is a placeholder for now
    }

    pub fn get_buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }

    pub fn get_buffers_mut(&mut self) -> &mut Vec<Buffer> {
        &mut self.buffers
    }
}
