use crate::state::EditorState;

impl EditorState {
    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.parent
            .get(&node_id)
            .and_then(|parent| parent.as_ref().copied())
    }
}
