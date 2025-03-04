use crate::EditorState;

pub trait EditorTransform: Send + Sync {
    fn transform(&self, state: &mut EditorState) -> bool;
}
