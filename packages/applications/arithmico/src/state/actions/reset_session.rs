use leptos::logging::error;
use web_state::StateMutation;

use crate::state::State;

use super::ClearInputAction;

pub struct ResetSessionAction;

impl ResetSessionAction {
    pub fn new() -> Self {
        Self
    }
}

impl StateMutation<State> for ResetSessionAction {
    fn apply(&self, state: &mut State) {
        state.log.clear();
        if let Err(err) = state
            .input_editor_state
            .execute_command(Box::new(ClearInputAction::new()))
        {
            error!("Failed to clear input field: {}", err);
        }
        state.current_output = None;
    }
}
