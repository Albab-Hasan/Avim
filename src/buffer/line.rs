// Line representation with metadata
pub struct Line {
    content: String,
    dirty: bool,
}

impl Line {
    pub fn new(content: String) -> Self {
        Self {
            content,
            dirty: false,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn insert(&mut self, pos: usize, ch: char) {
        self.content.insert(pos, ch);
        self.dirty = true;
    }

    pub fn remove(&mut self, pos: usize) -> char {
        self.dirty = true;
        self.content.remove(pos)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

