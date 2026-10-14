use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_whitespaces(
        &self,
        node_id: usize,
    ) -> Result<Vec<EditorWhitespace>, crate::Error> {
        let node = self.get_leaf_node(node_id)?;
        Ok(node
            .get_whitespaces()
            .into_iter()
            .map(|(start_offset, end_offset)| EditorWhitespace {
                node_id,
                start_offset,
                end_offset,
            })
            .collect())
    }
}
