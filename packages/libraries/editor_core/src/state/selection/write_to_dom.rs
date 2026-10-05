use leptos::prelude::document;
use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    fn get_selection_data(&self) -> Option<(Node, usize, Node, usize)> {
        let range = self.get_selection()?;
        let focus_node = self.get_dom_node(range.get_focus().get_node_id())?;
        let anchor_node =
            self.get_dom_node(range.get_anchor().get_node_id())?;
        Some((
            anchor_node,
            range.get_anchor().get_offset(),
            focus_node,
            range.get_focus().get_offset(),
        ))
    }

    pub fn write_selection_to_dom(&self) -> Result<(), crate::Error> {
        let selection = document()
            .get_selection()?
            .ok_or(crate::Error::MissingSelection)?;

        match self.get_selection_data() {
            Some((anchor_node, anchor_offset, focus_node, focus_offset)) => {
                selection.set_base_and_extent(
                    &anchor_node,
                    anchor_offset as u32,
                    &focus_node,
                    focus_offset as u32,
                )?;
            }
            None => selection.remove_all_ranges()?,
        }

        Ok(())
    }
}
