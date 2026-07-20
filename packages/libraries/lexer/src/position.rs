#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub byte_index: usize,
    pub char_index: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.byte_index.cmp(&other.byte_index)
    }
}

impl Position {
    pub fn new(byte_index: usize, char_index: usize) -> Self {
        Self {
            byte_index,
            char_index,
        }
    }
}
