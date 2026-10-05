use crate::{EditorNode, EditorState};

impl EditorState {
    pub fn insert_node_after(
        &mut self,
        node: EditorNode,
        sibling_id: usize,
    ) -> Result<usize, crate::Error> {
        let parent_id = self
            .get_parent_id(sibling_id)?
            .ok_or_else(|| crate::Error::ParentNodeNotFound)?;
        let sibling_position = self.get_child_position(sibling_id)?;
        self.insert_node(node, Some(parent_id), Some(sibling_position + 1))
    }
}
