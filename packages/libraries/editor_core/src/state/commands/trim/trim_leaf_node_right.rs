use crate::EditorState;

impl EditorState {
    pub fn trim_leaf_node_right(
        &mut self,
        node_id: usize,
        position: usize,
    ) -> Option<()> {
        let node = self.get_leaf_node(node_id)?;
        if position > 0 {
            self.replace_node(node_id, node.slice(position, node.length()));
        } else {
            self.delete_node(node_id);
        }
        Some(())
    }
}
