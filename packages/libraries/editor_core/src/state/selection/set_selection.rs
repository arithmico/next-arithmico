use crate::state::EditorState;

use super::{SelectionRange, selection_range::AbsoluteSelectionRange};

impl EditorState {
    pub fn set_selection(&mut self, range: SelectionRange) {
        self.selection = Some(range);
    }

    pub fn set_absolute_selection(
        &mut self,
        selection: AbsoluteSelectionRange,
    ) {
        if let Some(selection) =
            self.convert_absolute_selection_to_selection(selection)
        {
            self.set_selection(selection);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }
}
