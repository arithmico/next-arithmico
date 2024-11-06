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
            let focus_node_id = selection.get_focus().get_node_id();
            let anchor_node_id = selection.get_anchor().get_node_id();
            let focus_offset = selection.get_focus().get_offset();
            let anchor_offset = selection.get_anchor().get_offset();
            let nodes_between =
                state.get_all_node_ids_between(focus_node_id, anchor_node_id);
            state.delete_many_nodes(nodes_between.into_iter().collect());

            if focus_node_id == anchor_node_id {
                let node = state.get_leaf_node(focus_node_id)?;
                let start_offset = focus_offset.min(anchor_offset);
                let stop_offset = focus_offset.max(anchor_offset);
                let left_node = node.slice(0, start_offset);
                let right_node = node.slice(stop_offset, node.length());
                state.replace_node(focus_node_id, left_node);
                state.insert_node_after(right_node, focus_node_id);
                state.set_selection(SelectionRange::new_at(
                    focus_node_id,
                    start_offset,
                ));
            } else {
                let is_selection_left_to_right = !state
                    .is_node_before(anchor_node_id, focus_node_id)
                    .unwrap();

                if is_selection_left_to_right {
                    state.trim_leaf_node_left(anchor_node_id, anchor_offset);
                    state.trim_leaf_node_right_with_delete_option(
                        focus_node_id,
                        focus_offset,
                        false,
                    );
                    state.set_selection(SelectionRange::new_at(
                        focus_node_id,
                        0,
                    ));
                } else {
                    state.trim_leaf_node_right(anchor_node_id, anchor_offset);
                    state.trim_leaf_node_left_with_delete_option(
                        focus_node_id,
                        focus_offset,
                        false,
                    );
                    state.set_selection(SelectionRange::new_at(
                        focus_node_id,
                        focus_offset,
                    ));
                }
            }
        }

        Some(())
    }
}
