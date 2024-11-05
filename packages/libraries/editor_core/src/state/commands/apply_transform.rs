use crate::EditorState;

impl EditorState {
    pub fn apply_transforms(&mut self) {
        for transform in self.transforms.clone() {
            transform.transform(self);
        }
    }
}
