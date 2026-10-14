use crate::EditorState;

impl EditorState {
    pub fn get_all_leaf_node_ids_after(
        &self,
        node_id: usize,
    ) -> Result<Vec<usize>, crate::Error> {
        let mut all_nodes_ids = self.get_all_node_ids_in_order();
        let position = all_nodes_ids
            .iter()
            .position(|id| *id == node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)?;

        Ok(all_nodes_ids
            .split_off(position + 1)
            .into_iter()
            .filter(|node_id| self.is_leaf_node(*node_id))
            .collect())
    }
}
