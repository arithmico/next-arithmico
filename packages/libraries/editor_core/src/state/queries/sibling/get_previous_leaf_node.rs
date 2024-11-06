use crate::EditorState;

impl EditorState {
    pub fn get_previous_leaf_node(&self, node_id: usize) -> Option<usize> {
        let leaf_nodes = self
            .get_all_node_ids_in_order()
            .into_iter()
            .filter(|node_id| self.is_leaf_node(*node_id))
            .collect::<Vec<_>>();

        let position = leaf_nodes
            .iter()
            .position(|leaf_node_id| *leaf_node_id == node_id)?;

        let previous_position = position.checked_sub(1)?;

        leaf_nodes.get(previous_position).copied()
    }
}
