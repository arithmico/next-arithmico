use crate::core::state::EditorState;

impl EditorState {
    pub fn get_all_node_ids_between(
        &self,
        from_node_id: usize,
        to_node_id: usize,
    ) -> Vec<usize> {
        let node_ids = self.get_all_node_ids_in_order();
        let from = node_ids
            .iter()
            .position(|node_id| *node_id == from_node_id)
            .expect("from position");
        let to = node_ids
            .iter()
            .position(|node_id| *node_id == to_node_id)
            .expect("to position");
        let start = from.min(to);
        let end = from.max(to);
        node_ids
            .into_iter()
            .enumerate()
            .filter_map(|(position, node_id)| {
                if position > start && position < end {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect()
    }
}
