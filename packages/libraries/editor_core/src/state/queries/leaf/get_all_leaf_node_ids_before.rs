use crate::EditorState;

impl EditorState {
    pub fn get_all_leaf_node_ids_before(&self, node_id: usize) -> Vec<usize> {
        let mut all_nodes_ids = self.get_all_node_ids_in_order();
        let position = all_nodes_ids
            .iter()
            .position(|id| *id == node_id)
            .expect("position");
        all_nodes_ids.truncate(position);
        all_nodes_ids
            .into_iter()
            .filter(|node_id| self.is_leaf_node(*node_id))
            .collect()
    }
}
