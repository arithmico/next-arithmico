use crate::core::{state::node_mapping::NodeMapping, SelectionRange};

impl NodeMapping {
    pub fn get_selection(&self) -> Option<&SelectionRange> {
        self.selection.as_ref()
    }
}
