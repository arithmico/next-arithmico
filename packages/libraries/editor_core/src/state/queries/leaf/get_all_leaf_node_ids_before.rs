use crate::EditorState;

impl EditorState {
    pub fn get_all_leaf_node_ids_before(
        &self,
        node_id: usize,
    ) -> Result<Vec<usize>, crate::Error> {
        let mut all_nodes_ids = self.get_all_node_ids_in_order();
        let position = all_nodes_ids
            .iter()
            .position(|id| *id == node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)?;
        all_nodes_ids.truncate(position);
        Ok(all_nodes_ids
            .into_iter()
            .filter(|node_id| self.is_leaf_node(*node_id))
            .collect())
    }
}
