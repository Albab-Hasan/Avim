mod split;

pub use split::{Split, SplitType, WindowLayout};

pub struct Window {
    pub buffer_id: usize,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub viewport_offset: usize,
}

impl Window {
    pub fn new(buffer_id: usize) -> Self {
        Self {
            buffer_id,
            cursor_line: 0,
            cursor_col: 0,
            viewport_offset: 0,
        }
    }
}

