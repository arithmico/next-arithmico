use crate::state::EditorState;

impl EditorState {
    pub fn get_children_count(
        &self,
        node_id: usize,
    ) -> Result<Option<usize>, crate::Error> {
        Ok(self
            .children
            .get(&node_id)
            .map(|children| children.as_ref())
            .ok_or_else(|| crate::Error::NodeNotFound)?
            .map(|children| children.len()))
    }
}
