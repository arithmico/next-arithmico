use crate::EditorState;

impl EditorState {
    pub fn trim_leaf_node_right(
        &mut self,
        node_id: usize,
        position: usize,
    ) -> Option<()> {
        self.trim_leaf_node_right_with_delete_option(node_id, position, true)
    }

    pub fn trim_leaf_node_right_with_delete_option(
        &mut self,
        node_id: usize,
        position: usize,
        delete_empty_node: bool,
    ) -> Option<()> {
        let node = self.get_leaf_node(node_id)?;
        if position == 0 && delete_empty_node {
            self.delete_node(node_id);
        } else {
            self.replace_node(node_id, node.slice(position, node.length()));
        }
        Some(())
    }
}
