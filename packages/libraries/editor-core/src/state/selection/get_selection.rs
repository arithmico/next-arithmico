use crate::state::EditorState;

use super::SelectionRange;

impl EditorState {
    pub fn get_selection(&self) -> Option<SelectionRange> {
        self.selection.clone()
    }

    pub fn get_selection_or_err(&self) -> Result<SelectionRange, crate::Error> {
        self.get_selection()
            .ok_or_else(|| crate::Error::MissingSelection)
    }
}
