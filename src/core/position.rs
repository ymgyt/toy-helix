/// Represents a single point in a text buffer. Zero indexed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
