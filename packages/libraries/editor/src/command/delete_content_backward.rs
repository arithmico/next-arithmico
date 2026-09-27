use editor_core::{EditorCommand, selection::SelectionRange};

#[derive(Debug, Default)]
pub struct DeleteContentBackwardCommand;

impl EditorCommand for DeleteContentBackwardCommand {
    fn apply(&self, state: &mut editor_core::EditorState) -> Option<()> {
        let selection = state.get_selection()?;

        if selection.is_collapsed() {
            let node_id = selection.get_focus().get_node_id();
            let offset = selection.get_focus().get_offset();
            if offset > 0 {
                let node = state.get_leaf_node(node_id)?;
                let left_node = node.slice(0, offset - 1);
                let right_node = node.slice(offset, node.length());
                state.replace_node(node_id, left_node);
                state.insert_node_after(right_node, node_id);
                state
                    .set_selection(SelectionRange::new_at(node_id, offset - 1));
            } else {
                // TODO: find previous **non-empty** leaf node
                // TODO: delete all empty leaf nodes between
                let previous_leaf_node_id =
                    state.get_previous_leaf_node(node_id)?;
                let previous_leaf_node =
                    state.get_leaf_node(previous_leaf_node_id)?;
                let end_pos = previous_leaf_node.length().saturating_sub(1);
                let new_node = previous_leaf_node.slice(0, end_pos);
                state.replace_node(previous_leaf_node_id, new_node);
                state.set_selection(SelectionRange::new_at(
                    previous_leaf_node_id,
                    end_pos,
                ));
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Some(())
    }
}
