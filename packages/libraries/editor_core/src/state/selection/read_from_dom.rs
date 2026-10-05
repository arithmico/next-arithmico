use leptos::prelude::document;

use crate::state::EditorState;

use super::{SelectionRange, SelectionRangePoint};

impl EditorState {
    // TODO: consider Result<Option<...>>
    fn get_selection_from_dom(&self) -> Option<SelectionRange> {
        let selection = document().get_selection().ok()??;
        let focus_node_id =
            self.find_id_for_dom_node(&selection.focus_node()?)?;
        let anchor_node_id =
            self.find_id_for_dom_node(&selection.anchor_node()?)?;
        let focus_offset = selection.focus_offset() as usize;
        let anchor_offset = selection.anchor_offset() as usize;
        Some(SelectionRange::new(
            SelectionRangePoint::new(anchor_node_id, anchor_offset),
            SelectionRangePoint::new(focus_node_id, focus_offset),
        ))
    }

    pub fn read_selection_from_dom(&mut self) {
        self.selection = self.get_selection_from_dom();
    }
}
