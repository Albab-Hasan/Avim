mod normal;
mod insert;
mod visual;
pub mod command;

pub use normal::NormalMode;
pub use insert::InsertMode;
pub use visual::{VisualMode, VisualType};
pub use command::CommandMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual(VisualType),
    Command,
}

impl Mode {
    pub fn name(&self) -> &str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual(VisualType::Character) => "VISUAL",
            Mode::Visual(VisualType::Line) => "VISUAL LINE",
            Mode::Visual(VisualType::Block) => "VISUAL BLOCK",
            Mode::Command => "COMMAND",
        }
    }
}

