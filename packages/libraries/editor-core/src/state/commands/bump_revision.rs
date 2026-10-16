use crate::EditorState;

impl EditorState {
    pub fn bump_revision(&mut self) {
        self.revision += 1;
    }
}
