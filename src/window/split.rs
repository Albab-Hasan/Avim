#[derive(Debug, Clone, Copy)]
pub enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf { window_id: usize },
    Split { 
        split_type: SplitType,
        ratio: f32,
        children: Vec<Box<LayoutNode>>
    }
}

impl LayoutNode {
    pub fn is_leaf(&self) -> bool {
        matches!(self, LayoutNode::Leaf { .. })
    }

    pub fn is_split(&self) -> bool {
        matches!(self, LayoutNode::Split { .. })
    }

    pub fn get_window_id(&self) -> Option<usize> {
        match self {
            LayoutNode::Leaf { window_id } => Some(*window_id),
            _ => None,
        }
    }

    pub fn get_split_info(&self) -> Option<(SplitType, f32, &[Box<LayoutNode>])> {
        match self {
            LayoutNode::Split { split_type, ratio, children } => {
                Some((*split_type, *ratio, children.as_slice()))
            }
            _ => None,
        }
    }
}

// Legacy structures for backward compatibility
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

