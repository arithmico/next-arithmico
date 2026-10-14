use std::sync::Arc;

use crate::{EditorState, EditorTransform};

impl EditorState {
    pub fn add_transform<T: EditorTransform + 'static>(
        &mut self,
        transform: T,
    ) {
        self.transforms.push(Arc::new(transform));
    }
}
