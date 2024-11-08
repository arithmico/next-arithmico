use crate::EditorState;

impl EditorState {
    pub fn get_node_id_and_offset_from_absolute_offset(
        &self,
        absolute_offset: usize,
    ) -> Option<(usize, usize)> {
        let all_leaf_node_ids = self.get_all_leaf_node_ids();
        let last_leaf_id = all_leaf_node_ids.last().copied();
        let mut pos = 0;
        for node_id in all_leaf_node_ids {
            let last_leaf_id = last_leaf_id.unwrap();
            let node = self.get_leaf_node(node_id)?;
            let length = node.length();
            if absolute_offset == pos {
                return Some((node_id, 0));
            }
            if length < 1 {
                continue;
            }

            if absolute_offset < pos + length {
                return Some((node_id, absolute_offset - pos));
            }
            if node_id == last_leaf_id && absolute_offset == pos + length {
                return Some((node_id, absolute_offset - pos));
            }

            pos += length
        }

        None
    }
}
