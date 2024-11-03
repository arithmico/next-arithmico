use crate::core::state::EditorState;

impl EditorState {
    pub fn is_parent_of(&self, parent_id: usize, child_id: usize) -> bool {
        let mut current_node_id = self.get_parent_id(child_id);
        while let Some(node_id) = current_node_id {
            if node_id == parent_id {
                return true;
            }
            current_node_id = self.get_parent_id(node_id);
        }
        false
    }
}
