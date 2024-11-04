use editor_core::{
    selection::SelectionRange, EditorCommand, EditorLeafNode, EditorState,
};

use crate::node::TextNode;

pub struct InsertTextCommand {
    text: String,
}

impl InsertTextCommand {
    pub fn new<T: ToString>(from: T) -> Self {
        Self {
            text: from.to_string(),
        }
    }

    fn insert_into_node(
        &self,
        state: &mut EditorState,
        node_id: usize,
        focus_offset: usize,
        anchor_offset: usize,
    ) -> Option<()> {
        let node = state.get_leaf_node(node_id)?.boxed_clone();
        let start_offset = focus_offset.min(anchor_offset);
        let end_offset = focus_offset.max(anchor_offset);

        if start_offset > 0 {
            state.insert_node_before(node.slice(0, start_offset), node_id);
        }

        state.replace_node(
            node_id,
            TextNode::new_with_content(&self.text).into_editor_node(),
        );
        state.set_selection(SelectionRange::new_at(node_id, self.text.len()));

        if end_offset < node.length() - 1 {
            state.insert_node_after(
                node.slice(end_offset, node.length()),
                node_id,
            );
        }

        Some(())
    }
}

impl EditorCommand for InsertTextCommand {
    fn apply(&self, state: &mut EditorState) {
        let Some(range) = state.get_selection() else {
            return;
        };
        let nodes_between = state.get_all_node_ids_between(
            range.get_anchor().get_node_id(),
            range.get_focus().get_node_id(),
        );
        state.delete_many_nodes(nodes_between.into_iter().collect());

        let focus_node_id = range.get_focus().get_node_id();
        let anchor_node_id = range.get_anchor().get_node_id();
        let focus_offset = range.get_focus().get_offset();
        let anchor_offset = range.get_anchor().get_offset();

        if focus_node_id == anchor_node_id {
            self.insert_into_node(
                state,
                focus_node_id,
                focus_offset,
                anchor_offset,
            );
        } else {
            let new_node =
                TextNode::new_with_content(&self.text).into_editor_node();
            let node_id = if state
                .is_node_before(anchor_node_id, focus_node_id)
                .unwrap()
            {
                // left to right selection
                let node_id = state.insert_node_before(new_node, focus_node_id);
                state.trim_leaf_node_left(anchor_node_id, anchor_offset);
                state.trim_leaf_node_right(focus_node_id, focus_offset);
                node_id
            } else {
                // right to left selection
                let node_id = state.insert_node_after(new_node, focus_node_id);
                state.trim_leaf_node_right(anchor_node_id, anchor_offset);
                state.trim_leaf_node_left(focus_node_id, focus_offset);
                node_id
            };
            state.set_selection(SelectionRange::new_at(
                node_id,
                self.text.len(),
            ));
        }
    }
}
