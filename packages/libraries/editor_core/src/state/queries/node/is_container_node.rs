use crate::state::EditorState;

impl EditorState {
    pub fn is_container_node(
        &self,
        node_id: usize,
    ) -> Result<bool, crate::Error> {
        Ok(self.get_node(node_id)?.supports_children())
    }
}
