use crate::state::EditorState;

impl EditorState {
    pub fn has_children(&self, node_id: usize) -> bool {
        self.children
            .get(&node_id)
            .and_then(|children| {
                children.as_ref().map(|children| !children.is_empty())
            })
            .unwrap_or(false)
    }

    pub fn is_empty_container(
        &self,
        node_id: usize,
    ) -> Result<bool, crate::Error> {
        if !self.is_container_node(node_id)? {
            return Ok(false);
        }
        Ok(self.get_children_ids(node_id).is_empty())
    }
}
