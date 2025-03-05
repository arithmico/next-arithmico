use editor::editor::EditorStateMutation;
use web_state::WebStateAction;

use crate::state::State;

pub struct UpdateInputEditorAction {
    command: EditorStateMutation,
}

impl UpdateInputEditorAction {
    pub fn new(f: EditorStateMutation) -> Self {
        Self { command: f }
    }
}

impl WebStateAction<State> for UpdateInputEditorAction {
    fn apply(&self, state: &mut State) {
        self.command.run(&mut state.input_editor_state);
    }
}
