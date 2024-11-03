use crate::state::EditorState;

impl EditorState {
    pub fn get_child_id_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> Option<usize> {
        self.get_children_ids(node_id).get(position).copied()
    }
}
