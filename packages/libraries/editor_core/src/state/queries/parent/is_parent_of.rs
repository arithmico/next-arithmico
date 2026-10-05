use crate::state::EditorState;

impl EditorState {
    pub fn is_parent_of(
        &self,
        parent_id: usize,
        child_id: usize,
    ) -> Result<bool, crate::Error> {
        let mut current_node_id = self.get_parent_id(child_id)?;
        while let Some(node_id) = current_node_id {
            if node_id == parent_id {
                return Ok(true);
            }
            current_node_id = self.get_parent_id(node_id)?;
        }
        Ok(false)
    }
}
