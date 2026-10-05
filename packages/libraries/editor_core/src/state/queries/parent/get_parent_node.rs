use crate::{EditorNode, state::EditorState};

#[allow(dead_code)]
impl EditorState {
    pub fn get_parent_node(
        &self,
        node_id: usize,
    ) -> Result<Option<&EditorNode>, crate::Error> {
        match self.get_parent_id(node_id)? {
            Some(parent_id) => Ok(Some(self.get_node(parent_id)?)),
            None => Ok(None),
        }
    }
}
