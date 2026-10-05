use crate::state::EditorState;

impl EditorState {
    pub fn get_child_position(
        &self,
        node_id: usize,
    ) -> Result<usize, crate::Error> {
        let parent_id = self
            .get_parent_id(node_id)?
            .ok_or_else(|| crate::Error::ParentNodeNotFound)?;
        let children = self.get_children_ids(parent_id);
        children
            .iter()
            .position(|child_id| child_id.eq(&node_id))
            .ok_or_else(|| crate::Error::NodeNotFound)
    }
}
