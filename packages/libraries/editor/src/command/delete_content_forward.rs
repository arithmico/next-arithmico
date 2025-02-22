use editor_core::{EditorCommand, selection::SelectionRange};

pub struct DeleteContentForwardCommand;

impl DeleteContentForwardCommand {
    pub fn new() -> Self {
        Self
    }
}

impl EditorCommand for DeleteContentForwardCommand {
    fn apply(&self, state: &mut editor_core::EditorState) -> Option<()> {
        let selection = state.get_selection()?;

        if selection.is_collapsed() {
            let node_id = selection.get_focus().get_node_id();
            let offset = selection.get_focus().get_offset();
            let node = state.get_leaf_node(node_id)?;
            if offset < node.length() {
                let left_node = node.slice(0, offset);
                let right_node = node.slice(offset + 1, node.length());
                state.replace_node(node_id, left_node);
                state.insert_node_after(right_node, node_id);
                state.set_selection(SelectionRange::new_at(node_id, offset));
            } else {
                // TODO: find next **non-empty** leaf node
                // TODO: delete all empty leaf nodes between
                let next_leaf_node_id = state.get_next_leaf_node(node_id)?;
                let next_leaf_node = state.get_leaf_node(next_leaf_node_id)?;
                let leaf_length = next_leaf_node.length();
                let new_node = next_leaf_node.slice(1, leaf_length);
                state.replace_node(next_leaf_node_id, new_node);
                state.set_selection(SelectionRange::new_at(
                    next_leaf_node_id,
                    0,
                ));
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Some(())
    }
}
