use crate::EditorState;

impl EditorState {
    pub fn get_previous_leaf_node(
        &self,
        node_id: usize,
    ) -> Result<Option<usize>, crate::Error> {
        let mut nodes_before = self.get_all_node_ids_in_order();
        let node_position = nodes_before
            .iter()
            .position(|id| *id == node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)?;
        nodes_before.truncate(node_position);
        Ok(nodes_before
            .into_iter()
            .rfind(|node_id| self.is_leaf_node(*node_id)))
    }

    pub fn get_previous_leaf_node_or_err(
        &self,
        node_id: usize,
    ) -> Result<usize, crate::Error> {
        self.get_previous_leaf_node(node_id)?
            .ok_or_else(|| crate::Error::NodeNotFound)
    }
}
