use crate::{EditorNode, state::EditorState};

#[allow(dead_code)]
impl EditorState {
    pub fn get_parent_node(&self, node_id: usize) -> Option<&EditorNode> {
        self.get_parent_id(node_id)
            .map(|parent_id| self.get_node(parent_id))
            .flatten()
    }
}
