use crate::core::state::EditorState;

use super::SelectionRange;

impl EditorState {
    pub fn get_selection(&self) -> Option<&SelectionRange> {
        self.selection.as_ref()
    }
}
