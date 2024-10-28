#[derive(Debug, Clone)]
pub struct SelectionRangePoint {
    node_id: usize,
    offset: usize,
}

impl SelectionRangePoint {
    pub fn node_id(&self) -> usize {
        self.node_id
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
