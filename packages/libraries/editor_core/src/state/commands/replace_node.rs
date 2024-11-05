use crate::{state::EditorState, EditorNode};

impl EditorState {
    pub fn replace_node(&mut self, node_id: usize, node: EditorNode) {
        self.editor_nodes.insert(node_id, node);
        self.dom_nodes.insert(node_id, None);
        self.modified_nodes.insert(node_id);
    }
}
