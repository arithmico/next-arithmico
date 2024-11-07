use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_all_whitespaces_ending_before(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Vec<EditorWhitespace> {
        let mut node_ids = self.get_all_leaf_node_ids_before(node_id);
        if offset > 0 {
            node_ids.push(node_id);
        }
        let mut result = Vec::<EditorWhitespace>::new();

        for (index, id) in node_ids.iter().copied().enumerate() {
            if let Some(whitespaces) = self.get_whitespaces(id) {
                for whitespace in whitespaces {
                    if whitespace.node_id == node_id {
                        if whitespace.end_offset < offset {
                            result.push(whitespace);
                        }
                    } else {
                        let node_length =
                            self.get_leaf_node(id).expect("leaf node").length();
                        if index + 1 == node_ids.len()
                            && offset == 0
                            && whitespace.end_offset + 1 == node_length
                        {
                            continue;
                        }
                        result.push(whitespace);
                    }
                }
            }
        }

        result
    }
}
