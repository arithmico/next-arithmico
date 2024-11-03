use crate::core::state::EditorState;

impl EditorState {
    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.parent
            .get(&node_id)
            .map(|parent| parent.as_ref().copied())
            .flatten()
    }
}
