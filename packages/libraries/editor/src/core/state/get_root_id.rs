use super::EditorState;

impl EditorState {
    pub fn get_root_id(&self) -> usize {
        self.root_node_id
    }
}
