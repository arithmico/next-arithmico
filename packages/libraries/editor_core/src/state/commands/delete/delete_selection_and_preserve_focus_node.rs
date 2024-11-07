use crate::{selection::SelectionRange, EditorState};

impl EditorState {
    pub fn delete_selection_and_preserve_focus_node(&mut self) -> Option<()> {
        let selection = self.get_selection()?;

        if selection.is_collapsed() {
            return None;
        }

        let focus_node_id = selection.get_focus().get_node_id();
        let anchor_node_id = selection.get_anchor().get_node_id();
        let focus_offset = selection.get_focus().get_offset();
        let anchor_offset = selection.get_anchor().get_offset();
        let nodes_between = self
            .get_all_node_ids_between(focus_node_id, anchor_node_id)
            .into_iter()
            .filter(|node_id| {
                !self.is_parent_of(*node_id, focus_node_id)
                    && !self.is_parent_of(*node_id, anchor_node_id)
            });
        self.delete_many_nodes(nodes_between.collect());

        if focus_node_id == anchor_node_id {
            let node = self.get_leaf_node(focus_node_id)?;
            let start_offset = focus_offset.min(anchor_offset);
            let stop_offset = focus_offset.max(anchor_offset);
            let left_node = node.slice(0, start_offset);
            let right_node = node.slice(stop_offset, node.length());
            self.replace_node(focus_node_id, left_node);
            self.insert_node_after(right_node, focus_node_id);
            self.set_selection(SelectionRange::new_at(
                focus_node_id,
                start_offset,
            ));
        } else {
            let is_selection_left_to_right =
                self.is_node_before(anchor_node_id, focus_node_id).unwrap();

            if is_selection_left_to_right {
                self.trim_leaf_node_left(anchor_node_id, anchor_offset);
                self.trim_leaf_node_right_with_delete_option(
                    focus_node_id,
                    focus_offset,
                    false,
                );
                self.set_selection(SelectionRange::new_at(focus_node_id, 0));
            } else {
                self.trim_leaf_node_right(anchor_node_id, anchor_offset);
                self.trim_leaf_node_left_with_delete_option(
                    focus_node_id,
                    focus_offset,
                    false,
                );
                self.set_selection(SelectionRange::new_at(
                    focus_node_id,
                    focus_offset,
                ));
            }
        }

        Some(())
    }
}
