use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_all_whitespaces_starting_after(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Vec<EditorWhitespace> {
        let mut end_selected =
            self.get_leaf_node(node_id).expect("leaf node").length() == offset;

        let mut entered_node = false;
        let mut cut_off_pos = None;
        let mut whitespaces = self.get_all_whitespaces();
        for (pos, whitepsace) in whitespaces.iter().enumerate() {
            if whitepsace.node_id != node_id && entered_node {
                end_selected = false;
                continue;
            }
            if whitepsace.node_id != node_id && entered_node && !end_selected {
                cut_off_pos = Some(pos);
                break;
            }
            if whitepsace.node_id == node_id {
                entered_node = true;
                if whitepsace.start_offset > offset {
                    cut_off_pos = Some(pos);
                    break;
                }
            }
        }
        if let Some(cut_off_pos) = cut_off_pos {
            whitespaces.drain(..cut_off_pos).for_each(drop);
            whitespaces
        } else {
            Vec::new()
        }
    }
}
