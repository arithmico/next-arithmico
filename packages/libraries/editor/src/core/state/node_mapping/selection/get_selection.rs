use crate::core::{state::node_mapping::EditorState, SelectionRange};

impl EditorState {
    pub fn get_selection(&self) -> Option<&SelectionRange> {
        self.selection.as_ref()
    }
}
