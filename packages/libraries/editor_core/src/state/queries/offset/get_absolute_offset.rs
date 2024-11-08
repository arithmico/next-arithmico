use crate::EditorState;

impl EditorState {
    pub fn get_absolute_offset(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Option<usize> {
        if !self.is_leaf_node(node_id) {
            return None;
        }
        let previous_offset = self
            .get_all_leaf_node_ids_before(node_id)
            .into_iter()
            .map(|node_id| {
                let node = self.get_leaf_node(node_id).expect("leaf node");
                node.length()
            })
            .fold(0, |left, right| left + right);

        Some(previous_offset + offset)
    }
}
