use super::SelectionRangePoint;

#[derive(Debug, Clone)]
pub struct SelectionRange {
    anchor: SelectionRangePoint,
    focus: SelectionRangePoint,
}

impl SelectionRange {
    pub fn anchor(&self) -> &SelectionRangePoint {
        &self.anchor
    }

    pub fn focus(&self) -> &SelectionRangePoint {
        &self.focus
    }
}
