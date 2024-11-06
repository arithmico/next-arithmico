use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_all_whitespaces(&self) -> Vec<EditorWhitespace> {
        let mut whitespaces = Vec::new();
        for node_id in self.get_all_node_ids_in_order() {
            if let Some(mut node_whitespaces) = self.get_whitespaces(node_id) {
                whitespaces.append(&mut node_whitespaces);
            }
        }
        whitespaces
    }
}
