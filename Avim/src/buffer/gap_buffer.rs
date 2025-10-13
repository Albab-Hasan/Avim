// Gap buffer implementation for efficient text editing
pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new() -> Self {
        const INITIAL_GAP_SIZE: usize = 16;
        Self {
            buffer: vec!['\0'; INITIAL_GAP_SIZE],
            gap_start: 0,
            gap_end: INITIAL_GAP_SIZE,
        }
    }

    pub fn from_string(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let gap_size = 16.max(len / 2);
        
        let mut buffer = Vec::with_capacity(len + gap_size);
        buffer.extend_from_slice(&chars);
        buffer.extend(vec!['\0'; gap_size]);

        Self {
            buffer,
            gap_start: len,
            gap_end: len + gap_size,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn grow_gap(&mut self) {
        let gap_size = self.gap_end - self.gap_start;
        let new_gap_size = gap_size * 2;
        let additional = new_gap_size - gap_size;

        self.buffer.splice(self.gap_end..self.gap_end, vec!['\0'; additional]);
        self.gap_end += additional;
    }

    pub fn move_gap(&mut self, pos: usize) {
        if pos < self.gap_start {
            let distance = self.gap_start - pos;
            self.buffer.copy_within(pos..self.gap_start, self.gap_end - distance);
            self.gap_start = pos;
            self.gap_end -= distance;
        } else if pos > self.gap_start {
            let distance = pos - self.gap_start;
            self.buffer.copy_within(self.gap_end..self.gap_end + distance, self.gap_start);
            self.gap_start += distance;
            self.gap_end += distance;
        }
    }

    pub fn insert(&mut self, ch: char) {
        if self.gap_start == self.gap_end {
            self.grow_gap();
        }
        self.buffer[self.gap_start] = ch;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_end < self.buffer.len() {
            self.gap_end += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.len());
        result.extend(&self.buffer[..self.gap_start]);
        result.extend(&self.buffer[self.gap_end..]);
        result
    }
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self::new()
    }
}

