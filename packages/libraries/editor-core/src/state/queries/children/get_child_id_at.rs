use crate::state::EditorState;

impl EditorState {
    pub fn get_child_id_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> Result<usize, crate::Error> {
        self.get_children_ids(node_id)
            .get(position)
            .copied()
            .ok_or_else(|| crate::Error::NodeNotFound)
    }
}
