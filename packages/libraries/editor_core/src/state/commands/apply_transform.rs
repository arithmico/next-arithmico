use crate::EditorState;

impl EditorState {
    pub fn apply_transforms(&mut self) {
        loop {
            let mut modified = false;
            for transform in self.transforms.clone() {
                modified = modified || transform.transform(self);
            }
            if !modified {
                break;
            }
        }
    }
}
