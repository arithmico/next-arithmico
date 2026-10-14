use crate::state::EditorState;

impl EditorState {
    pub fn is_leaf_node(&self, node_id: usize) -> bool {
        if let Ok(node) = self.get_node(node_id) {
            !node.supports_children()
        } else {
            false
        }
    }
}
