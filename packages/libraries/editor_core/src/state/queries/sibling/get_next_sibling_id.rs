use crate::state::EditorState;

impl EditorState {
    pub fn get_next_sibling_id(&self, node_id: usize) -> Option<usize> {
        let position = self.get_child_position(node_id)?;
        let parent_id = self.get_parent_id(node_id)?;
        self.get_child_id_at(parent_id, position + 1)
    }
}
