use crate::EditorState;

impl EditorState {
    pub fn trim_leaf_node_left(
        &mut self,
        node_id: usize,
        position: usize,
    ) -> Option<()> {
        self.trim_leaf_node_left_with_delete_option(node_id, position, true)
    }

    pub fn trim_leaf_node_left_with_delete_option(
        &mut self,
        node_id: usize,
        position: usize,
        delete_empty: bool,
    ) -> Option<()> {
        let node = self.get_leaf_node(node_id)?;
        if position + 1 == node.length() && delete_empty {
            self.delete_node(node_id);
        } else {
            self.replace_node(node_id, node.slice(0, position));
        }
        Some(())
    }
}
