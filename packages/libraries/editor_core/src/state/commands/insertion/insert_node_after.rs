use crate::{EditorNode, EditorState};

impl EditorState {
    pub fn insert_node_after(
        &mut self,
        node: EditorNode,
        sibling_id: usize,
    ) -> usize {
        let parent_id = self.get_parent_id(sibling_id).expect("parent_id");
        let sibling_position = self
            .get_child_position(sibling_id)
            .expect("sibling position");
        self.insert_node(node, Some(parent_id), Some(sibling_position + 1))
    }
}
