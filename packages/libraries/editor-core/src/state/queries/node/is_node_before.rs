use crate::EditorState;

impl EditorState {
    pub fn is_node_before(
        &self,
        node_id: usize,
        other_node_id: usize,
    ) -> Result<bool, crate::Error> {
        let ordered_nodes = self.get_all_node_ids_in_order();
        let node_position = ordered_nodes
            .iter()
            .position(|id| id.eq(&node_id))
            .ok_or_else(|| crate::Error::NodeNotFound)?;
        let other_node_position = ordered_nodes
            .iter()
            .position(|id| id.eq(&other_node_id))
            .ok_or_else(|| crate::Error::NodeNotFound)?;

        Ok(node_position < other_node_position)
    }
}
