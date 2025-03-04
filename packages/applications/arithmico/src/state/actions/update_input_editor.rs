use editor_core::EditorState;
use web_state::WebStateAction;

use crate::state::State;

pub struct UpdateInputEditor {
    command: Box<dyn Fn(&mut EditorState)>,
}

impl UpdateInputEditor {
    pub fn new(f: Box<dyn Fn(&mut EditorState)>) -> Self {
        Self { command: f }
    }
}

impl WebStateAction<State> for UpdateInputEditor {
    fn apply(&self, state: &mut State) {
        (self.command)(&mut state.input_editor_state);
    }
}
