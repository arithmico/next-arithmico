#[derive(Debug, Clone)]
pub struct SelectionRangePoint {
    node_id: usize,
    offset: usize,
}

#[allow(dead_code)]
impl SelectionRangePoint {
    pub fn new(node_id: usize, offset: usize) -> Self {
        Self { node_id, offset }
    }

    pub fn get_node_id(&self) -> usize {
        self.node_id
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }
}
