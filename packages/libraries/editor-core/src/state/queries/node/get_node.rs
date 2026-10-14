use crate::{EditorNode, state::EditorState};

impl EditorState {
    pub fn get_node(
        &self,
        node_id: usize,
    ) -> Result<&EditorNode, crate::Error> {
        self.editor_nodes
            .get(&node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)
    }
}
