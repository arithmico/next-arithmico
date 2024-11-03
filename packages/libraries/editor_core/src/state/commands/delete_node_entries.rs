use crate::state::EditorState;

impl EditorState {
    pub fn delete_node_entries(&mut self, node_id: usize) {
        self.editor_nodes.remove(&node_id);
        self.dom_nodes.remove(&node_id);
        self.children.remove(&node_id);
        self.parent.remove(&node_id);
    }
}
