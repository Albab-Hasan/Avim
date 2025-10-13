#[derive(Debug, Clone, Copy)]
pub enum SplitType {
    Horizontal,
    Vertical,
}

pub struct Split {
    split_type: SplitType,
    ratio: f32,
}

impl Split {
    pub fn new(split_type: SplitType, ratio: f32) -> Self {
        Self { split_type, ratio }
    }

    pub fn split_type(&self) -> SplitType {
        self.split_type
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }
}

pub struct WindowLayout {
    splits: Vec<Split>,
}

impl WindowLayout {
    pub fn new() -> Self {
        Self { splits: Vec::new() }
    }

    pub fn add_split(&mut self, split: Split) {
        self.splits.push(split);
    }

    pub fn splits(&self) -> &[Split] {
        &self.splits
    }
}

impl Default for WindowLayout {
    fn default() -> Self {
        Self::new()
    }
}

