use crate::{state::EditorState, EditorNode};

impl EditorState {
    pub fn get_node(&self, node_id: usize) -> Option<&Box<dyn EditorNode>> {
        self.editor_nodes.get(&node_id)
    }
}
