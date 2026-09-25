use crate::state::EditorState;

impl EditorState {
    pub fn get_children_ids(&self, node_id: usize) -> Vec<usize> {
        self.children
            .get(&node_id)
            .and_then(|children| children.as_ref().cloned())
            .unwrap_or_default()
    }
}
