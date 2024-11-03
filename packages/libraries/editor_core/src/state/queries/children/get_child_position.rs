use crate::state::EditorState;

impl EditorState {
    pub fn get_child_position(&self, node_id: usize) -> Option<usize> {
        let parent_id = self.get_parent_id(node_id)?;
        let children = self.get_children_ids(parent_id);
        children.iter().position(|child_id| child_id.eq(&node_id))
    }
}
