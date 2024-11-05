use std::rc::Rc;

use crate::{EditorState, EditorTransform};

impl EditorState {
    pub fn add_transform<T: EditorTransform + 'static>(
        &mut self,
        transform: T,
    ) {
        self.transforms.push(Rc::new(transform));
    }
}
