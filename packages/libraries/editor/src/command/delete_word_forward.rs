use editor_core::{selection::SelectionRange, EditorCommand};

pub struct DeleteWordForwardCommand;

impl DeleteWordForwardCommand {
    pub fn new() -> Self {
        Self
    }
}

impl EditorCommand for DeleteWordForwardCommand {
    fn apply(&self, state: &mut editor_core::EditorState) -> Option<()> {
        let selection = state.get_selection()?;

        if selection.is_collapsed() {
            let node_id = selection.get_focus().get_node_id();
            let offset = selection.get_focus().get_offset();
            let node = state.get_leaf_node(node_id)?;
            let whitespaces =
                state.get_all_whitespaces_starting_after(node_id, offset);

            if let Some(whitespace) = whitespaces.first() {
                if whitespace.node_id == node_id {
                    let target_pos =
                        (whitespace.start_offset).min(node.length());

                    let left_node = node.slice(0, offset);
                    let right_node = node.slice(target_pos, node.length());
                    state.replace_node(node_id, left_node);
                    let new_node_id =
                        state.insert_node_after(right_node, node_id);
                    state.set_selection(SelectionRange::new_at(new_node_id, 0));
                } else {
                    let end_node = state.get_leaf_node(whitespace.node_id)?;
                    let target_pos =
                        (whitespace.start_offset).min(end_node.length());
                    let left_node = node.slice(0, offset);
                    let right_node =
                        end_node.slice(target_pos, end_node.length());
                    let nodes_to_delete = state
                        .get_all_node_ids_between(whitespace.node_id, node_id);

                    state.delete_many_nodes(
                        nodes_to_delete.into_iter().collect(),
                    );
                    state.replace_node(node_id, left_node);
                    state.replace_node(whitespace.node_id, right_node);
                    state.set_selection(SelectionRange::new_at(
                        whitespace.node_id,
                        0,
                    ));
                }
            } else {
                let all_leaf_nodes_after =
                    state.get_all_leaf_node_ids_after(node_id);
                state.replace_node(node_id, node.slice(0, offset));
                state.set_selection(SelectionRange::new_at(node_id, offset));
                state.delete_many_nodes(
                    all_leaf_nodes_after.into_iter().collect(),
                );
            }
        } else {
            state.delete_selection_and_preserve_focus_node()?;
        }

        Some(())
    }
}
