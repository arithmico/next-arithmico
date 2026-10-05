use crate::state::EditorState;

impl EditorState {
    pub fn get_previous_sibling_id(
        &self,
        node_id: usize,
    ) -> Result<Option<usize>, crate::Error> {
        let position = self.get_child_position(node_id)?;
        if position < 1 {
            return Ok(None);
        }
        let parent_id = self
            .get_parent_id(node_id)?
            .ok_or_else(|| crate::Error::ParentNodeNotFound)?;
        Ok(self.get_child_id_at(parent_id, position - 1))
    }
}
