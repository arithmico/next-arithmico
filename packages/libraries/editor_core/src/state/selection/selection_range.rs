use super::SelectionRangePoint;

#[derive(Debug, Clone)]
pub struct SelectionRange {
    anchor: SelectionRangePoint,
    focus: SelectionRangePoint,
}

#[allow(dead_code)]
impl SelectionRange {
    pub fn new(
        anchor: SelectionRangePoint,
        focus: SelectionRangePoint,
    ) -> Self {
        Self { anchor, focus }
    }

    pub fn new_at(node_id: usize, offset: usize) -> Self {
        Self {
            anchor: SelectionRangePoint::new(node_id, offset),
            focus: SelectionRangePoint::new(node_id, offset),
        }
    }

    pub fn get_anchor(&self) -> &SelectionRangePoint {
        &self.anchor
    }

    pub fn get_focus(&self) -> &SelectionRangePoint {
        &self.focus
    }

    pub fn is_collapsed(&self) -> bool {
        self.anchor.get_node_id() == self.focus.get_node_id()
            && self.anchor.get_offset() == self.focus.get_offset()
    }
}

#[derive(Debug, Clone)]
pub struct AbsoluteSelectionRange {
    focus: usize,
    anchor: usize,
}

impl AbsoluteSelectionRange {
    pub fn new(focus: usize, anchor: usize) -> Self {
        Self { focus, anchor }
    }

    pub fn focus(&self) -> usize {
        self.focus
    }

    pub fn anchor(&self) -> usize {
        self.anchor
    }
}
