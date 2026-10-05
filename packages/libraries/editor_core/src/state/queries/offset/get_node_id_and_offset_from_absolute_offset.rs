use crate::EditorState;

impl EditorState {
    pub fn get_node_id_and_offset_from_absolute_offset(
        &self,
        absolute_offset: usize,
    ) -> Result<(usize, usize), crate::Error> {
        let all_leaf_node_ids = self.get_all_leaf_node_ids();
        let last_leaf_id = all_leaf_node_ids.last().copied();
        let mut pos = 0;
        for node_id in all_leaf_node_ids {
            let last_leaf_id =
                last_leaf_id.ok_or_else(|| crate::Error::NodeNotFound)?;
            let node = self.get_leaf_node(node_id)?;
            let length = node.length();
            if absolute_offset == pos {
                return Ok((node_id, 0));
            }
            if length < 1 {
                continue;
            }

            if absolute_offset < pos + length {
                return Ok((node_id, absolute_offset - pos));
            }
            if node_id == last_leaf_id && absolute_offset == pos + length {
                return Ok((node_id, absolute_offset - pos));
            }

            pos += length
        }

        // TODO: should we add a better error variant?
        Err(crate::Error::NodeNotFound)
    }
}
