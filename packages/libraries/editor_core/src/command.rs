use super::state::EditorState;

pub trait EditorCommand {
    fn apply(&self, state: &mut EditorState) -> Option<()>;
}

impl<T: EditorCommand + 'static> From<T> for Box<dyn EditorCommand> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
