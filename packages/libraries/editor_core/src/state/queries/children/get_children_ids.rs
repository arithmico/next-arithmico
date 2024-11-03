use crate::state::EditorState;

impl EditorState {
    pub fn get_children_ids(&self, node_id: usize) -> Vec<usize> {
        self.children
            .get(&node_id)
            .map(|children| children.as_ref())
            .flatten()
            .map(|children| children.clone())
            .unwrap_or_else(|| vec![])
    }
}
