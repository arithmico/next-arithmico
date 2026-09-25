use crate::EditorState;

impl EditorState {
    pub fn get_previous_leaf_node(&self, node_id: usize) -> Option<usize> {
        let mut nodes_before = self.get_all_node_ids_in_order();
        let node_position =
            nodes_before.iter().position(|id| *id == node_id)?;
        nodes_before.truncate(node_position);
        nodes_before
            .into_iter()
            .rfind(|node_id| self.is_leaf_node(*node_id))
    }
}
