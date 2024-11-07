use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_all_whitespaces_starting_after(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Vec<EditorWhitespace> {
        let end_selected =
            self.get_leaf_node(node_id).expect("leaf node").length() == offset;

        let mut node_ids = self.get_all_leaf_node_ids_after(node_id);
        if !end_selected {
            node_ids.insert(0, node_id);
        }

        let mut result = Vec::<EditorWhitespace>::new();

        for (index, id) in node_ids.into_iter().enumerate() {
            if let Some(whitespaces) = self.get_whitespaces(id) {
                for whitespace in whitespaces {
                    if whitespace.node_id == node_id {
                        if whitespace.start_offset > offset {
                            result.push(whitespace);
                        }
                    } else {
                        if index == 0
                            && end_selected
                            && whitespace.start_offset == 0
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
