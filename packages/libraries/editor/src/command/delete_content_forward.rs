use editor_core::{EditorCommand, selection::SelectionRange};

#[derive(Debug, Default)]
pub struct DeleteContentForwardCommand;

impl EditorCommand for DeleteContentForwardCommand {
    fn apply(
        &self,
        state: &mut editor_core::EditorState,
    ) -> Result<(), editor_core::Error> {
        let selection = state.get_selection_or_err()?;

        if selection.is_collapsed() {
            let node_id = selection.get_focus().get_node_id();
            let offset = selection.get_focus().get_offset();
            let node = state.get_leaf_node(node_id)?;
            let len = node.length();

            if offset < node.length() {
                let char_len = node
                    .next_character_boundary(offset)
                    .map(|(a, b)| b - a)
                    .unwrap_or(1);

                let delete_end_pos = (offset + char_len).min(len);
                let left_node = node.slice(0, offset);
                let right_node = node.slice(delete_end_pos, len);

                state.replace_node(node_id, left_node)?;

                if delete_end_pos < len {
                    state.insert_node_after(right_node, node_id)?;
                }

                state.set_selection(SelectionRange::new_at(node_id, offset));
            } else {
                // TODO: find next **non-empty** leaf node
                // TODO: delete all empty leaf nodes between
                let Ok(next_leaf_node_id) =
                    state.get_next_leaf_node_or_err(node_id)
                else {
                    return Ok(());
                };
                let next_leaf_node = state.get_leaf_node(next_leaf_node_id)?;
                let leaf_length = next_leaf_node.length();
                let char_len = next_leaf_node
                    .next_character_boundary(0)
                    .map(|(a, b)| b - a)
                    .unwrap_or(1);
                let new_node = next_leaf_node.slice(char_len, leaf_length);
                state.replace_node(next_leaf_node_id, new_node)?;
                state.set_selection(SelectionRange::new_at(
                    next_leaf_node_id,
                    0,
                ));
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Ok(())
    }
}
