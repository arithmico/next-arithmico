use editor::editor::EditorStateMutation;
use leptos::logging::error;
use web_state::StateMutation;

use crate::state::State;

pub struct UpdateInputEditorAction {
    command: EditorStateMutation,
}

impl UpdateInputEditorAction {
    pub fn new(f: EditorStateMutation) -> Self {
        Self { command: f }
    }
}

impl StateMutation<State> for UpdateInputEditorAction {
    fn apply(&self, state: &mut State) {
        if let Err(err) = self.command.run(&mut state.input_editor_state) {
            error!("Failed to update input field: {}", err);
        };
    }
}
