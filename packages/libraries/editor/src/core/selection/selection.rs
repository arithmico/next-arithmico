use super::SelectionRange;

#[derive(Debug, Clone)]
pub struct Selection {
    ranges: Vec<SelectionRange>,
}

impl Selection {
    pub fn ranges(&self) -> &Vec<SelectionRange> {
        &self.ranges
    }
}
