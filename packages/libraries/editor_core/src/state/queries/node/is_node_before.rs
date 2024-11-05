use crate::EditorState;

impl EditorState {
    pub fn is_node_before(
        &self,
        node_id: usize,
        other_node_id: usize,
    ) -> Option<bool> {
        let ordered_nodes = self.get_all_node_ids_in_order();
        let node_position =
            ordered_nodes.iter().position(|id| id.eq(&node_id))?;
        let other_node_position =
            ordered_nodes.iter().position(|id| id.eq(&other_node_id))?;
        Some(node_position < other_node_position)
    }
}
