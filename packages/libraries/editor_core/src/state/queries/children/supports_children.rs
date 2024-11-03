use crate::state::EditorState;

impl EditorState {
    pub fn supports_children(&self, node_id: usize) -> bool {
        self.get_node(node_id).expect("node").supports_children()
    }
}
