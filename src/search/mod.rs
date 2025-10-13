use crate::buffer::Buffer;

#[derive(Clone)]
pub struct SearchState {
    pub query: String,
    pub forward: bool,
    pub matches: Vec<(usize, usize)>, // (line, col)
    pub current_match: Option<usize>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            forward: true,
            matches: Vec::new(),
            current_match: None,
        }
    }

    pub fn search(&mut self, buffer: &Buffer, query: &str, forward: bool) {
        self.query = query.to_string();
        self.forward = forward;
        self.matches.clear();
        self.current_match = None;

        if query.is_empty() {
            return;
        }

        // Find all matches
        for (line_idx, line) in (0..buffer.line_count()).filter_map(|i| buffer.get_line(i).map(|l| (i, l))) {
            let mut start = 0;
            while let Some(pos) = line[start..].find(query) {
                self.matches.push((line_idx, start + pos));
                start += pos + 1;
            }
        }

        if !self.matches.is_empty() {
            self.current_match = Some(0);
        }
    }

    pub fn next_match(&mut self) -> Option<(usize, usize)> {
        if self.matches.is_empty() {
            return None;
        }

        if let Some(current) = self.current_match {
            let next = if self.forward {
                (current + 1) % self.matches.len()
            } else {
                if current == 0 {
                    self.matches.len() - 1
                } else {
                    current - 1
                }
            };
            self.current_match = Some(next);
            Some(self.matches[next])
        } else {
            None
        }
    }

    pub fn prev_match(&mut self) -> Option<(usize, usize)> {
        if self.matches.is_empty() {
            return None;
        }

        if let Some(current) = self.current_match {
            let prev = if current == 0 {
                self.matches.len() - 1
            } else {
                current - 1
            };
            self.current_match = Some(prev);
            Some(self.matches[prev])
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<(usize, usize)> {
        self.current_match.and_then(|i| self.matches.get(i).copied())
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_match = None;
    }

    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

