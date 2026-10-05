use crate::EditorState;

impl EditorState {
    pub fn get_absolute_offset(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Result<usize, crate::Error> {
        if !self.is_leaf_node(node_id) {
            return Err(crate::Error::NotALeafNode);
        }
        let previous_offset = self
            .get_all_leaf_node_ids_before(node_id)?
            .into_iter()
            .filter_map(|node_id| {
                let node = self.get_leaf_node(node_id).ok()?;
                Some(node.length())
            })
            .sum::<usize>();

        Ok(previous_offset + offset)
    }
}
