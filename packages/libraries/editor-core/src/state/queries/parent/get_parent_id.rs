use crate::state::EditorState;

impl EditorState {
    pub fn get_parent_id(
        &self,
        node_id: usize,
    ) -> Result<Option<usize>, crate::Error> {
        self.parent
            .get(&node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)
            .copied()
    }
}
