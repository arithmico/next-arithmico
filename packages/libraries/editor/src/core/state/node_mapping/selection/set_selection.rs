use crate::core::{state::node_mapping::EditorState, SelectionRange};

impl EditorState {
    pub fn set_selection(&mut self, range: SelectionRange) {
        self.selection = Some(range);
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }
}
