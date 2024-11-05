use crate::state::EditorState;

impl EditorState {
    pub fn is_leaf_node(&self, node_id: usize) -> bool {
        !self.get_node(node_id).expect("node").supports_children()
    }
}
