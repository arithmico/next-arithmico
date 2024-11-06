use crate::EditorState;

use super::EditorWhitespace;

impl EditorState {
    pub fn get_all_whitespaces_ending_before(
        &self,
        node_id: usize,
        offset: usize,
    ) -> Vec<EditorWhitespace> {
        let mut entered_node = false;
        let mut cut_off_pos = None;
        let mut whitespaces = self.get_all_whitespaces();
        for (pos, whitepsace) in whitespaces.iter().enumerate() {
            if whitepsace.node_id != node_id && entered_node {
                cut_off_pos = Some(pos);
                break;
            }
            if whitepsace.node_id == node_id {
                entered_node = true;
                if whitepsace.end_offset >= offset {
                    cut_off_pos = Some(pos);
                    break;
                }
            }
        }
        if let Some(cut_off_pos) = cut_off_pos {
            whitespaces.truncate(cut_off_pos);
        }
        whitespaces
    }
}
