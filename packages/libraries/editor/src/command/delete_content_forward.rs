use editor_core::{selection::SelectionRange, EditorCommand};

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
                return None;
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Some(())
    }
}
