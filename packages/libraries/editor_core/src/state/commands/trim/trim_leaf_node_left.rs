use crate::EditorState;

impl EditorState {
    pub fn trim_leaf_node_left(
        &mut self,
        node_id: usize,
        position: usize,
    ) -> Option<()> {
        let node = self.get_leaf_node(node_id)?;
        if position + 1 < node.length() {
            self.replace_node(node_id, node.slice(0, position));
        } else {
            self.delete_node(node_id);
        }
        Some(())
    }
}
