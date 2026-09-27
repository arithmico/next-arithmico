use editor_core::{EditorCommand, selection::SelectionRange};

#[derive(Debug, Default)]
pub struct DeleteWordBackwardCommand;

impl EditorCommand for DeleteWordBackwardCommand {
    fn apply(&self, state: &mut editor_core::EditorState) -> Option<()> {
        let selection = state.get_selection()?;

        if selection.is_collapsed() {
            let node_id = selection.get_focus().get_node_id();
            let offset = selection.get_focus().get_offset();
            let node = state.get_leaf_node(node_id)?;
            let whitespaces = state.get_all_whitespaces_ending_before(
                node_id,
                offset.saturating_sub(1),
            );

            if let Some(whitespace) = whitespaces.last() {
                if whitespace.node_id == node_id {
                    let target_pos =
                        (whitespace.end_offset + 1).min(node.length());
                    let left_node = node.slice(0, target_pos);
                    let right_node = node.slice(offset, node.length());
                    state.replace_node(node_id, left_node);
                    state.insert_node_after(right_node, node_id);
                    state.set_selection(SelectionRange::new_at(
                        node_id, target_pos,
                    ));
                } else {
                    let end_node = state.get_leaf_node(whitespace.node_id)?;
                    let target_pos =
                        (whitespace.end_offset + 1).min(end_node.length());
                    let right_node = node.slice(offset, node.length());
                    let left_node = end_node.slice(0, target_pos);
                    let nodes_to_delete = state
                        .get_all_node_ids_between(whitespace.node_id, node_id);

                    state.delete_many_nodes(
                        nodes_to_delete.into_iter().collect(),
                    );
                    state.replace_node(node_id, right_node);
                    state.replace_node(whitespace.node_id, left_node);
                    state.set_selection(SelectionRange::new_at(
                        whitespace.node_id,
                        target_pos,
                    ));
                }
            } else {
                let all_leaf_nodes_before =
                    state.get_all_leaf_node_ids_before(node_id);

                state.replace_node(node_id, node.slice(offset, node.length()));
                state.set_selection(SelectionRange::new_at(node_id, 0));
                state.delete_many_nodes(
                    all_leaf_nodes_before.into_iter().collect(),
                );
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Some(())
    }
}
