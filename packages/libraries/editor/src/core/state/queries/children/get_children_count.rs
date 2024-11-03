use crate::core::state::EditorState;

impl EditorState {
    pub fn get_children_count(&self, node_id: usize) -> Option<usize> {
        Some(self.children.get(&node_id)?.as_ref()?.len())
    }
}
