use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::mode::Mode;

pub struct EditorState {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub mode: Mode,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
        }
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

