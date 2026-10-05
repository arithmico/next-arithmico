use crate::state::EditorState;

impl EditorState {
    pub fn get_all_node_ids_between(
        &self,
        from_node_id: usize,
        to_node_id: usize,
    ) -> Result<Vec<usize>, crate::Error> {
        let node_ids = self.get_all_node_ids_in_order();
        let from = node_ids
            .iter()
            .position(|node_id| *node_id == from_node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)?;
        let to = node_ids
            .iter()
            .position(|node_id| *node_id == to_node_id)
            .ok_or_else(|| crate::Error::NodeNotFound)?;

        let start = from.min(to) + 1;
        let end = from.max(to);

        if start > end {
            return Ok(vec![]);
        }

        let mut node_ids_between = vec![];
        for &node_id in &node_ids[start..end] {
            if self.is_parent_of(node_id, from_node_id)?
                || self.is_parent_of(node_id, to_node_id)?
            {
                continue;
            }
            node_ids_between.push(node_id);
        }

        Ok(node_ids_between)
    }
}
