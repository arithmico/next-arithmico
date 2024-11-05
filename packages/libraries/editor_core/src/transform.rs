use crate::EditorState;

pub trait EditorTransform {
    fn transform(&self, state: &mut EditorState);
}
