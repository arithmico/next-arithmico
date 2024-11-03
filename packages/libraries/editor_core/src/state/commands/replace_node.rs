use crate::{state::EditorState, EditorNode};

impl EditorState {
    pub fn replace_node(&mut self, node_id: usize, node: Box<dyn EditorNode>) {
        self.editor_nodes.insert(node_id, node);
        self.modified_nodes.insert(node_id);
    }
}
