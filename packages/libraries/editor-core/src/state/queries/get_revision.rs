use crate::EditorState;

impl EditorState {
    pub fn get_revision(&self) -> usize {
        self.revision
    }
}
