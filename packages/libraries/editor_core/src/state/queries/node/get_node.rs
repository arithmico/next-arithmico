use crate::{state::EditorState, EditorNode};

impl EditorState {
    pub fn get_node(&self, node_id: usize) -> Option<&EditorNode> {
        self.editor_nodes.get(&node_id)
    }
}
