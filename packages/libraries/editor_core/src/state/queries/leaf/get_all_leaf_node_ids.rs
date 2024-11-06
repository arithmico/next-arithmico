use crate::EditorState;

impl EditorState {
    pub fn get_all_leaf_node_ids(&self) -> Vec<usize> {
        self.get_all_node_ids_in_order()
            .into_iter()
            .filter(|node_id| self.is_leaf_node(*node_id))
            .collect()
    }
}
