use engine::Session;
use leptos::logging::error;
use web_state::WebStateAction;

use crate::state::State;

use super::ClearInputAction;

pub struct ResetSessionAction;

impl ResetSessionAction {
    pub fn new() -> Self {
        Self
    }
}

impl WebStateAction<State> for ResetSessionAction {
    fn apply(&self, state: &mut State) {
        state.session = Session::new();
        if let Err(err) = state
            .input_editor_state
            .execute_command(Box::new(ClearInputAction::new()))
        {
            error!("Failed to clear input field: {}", err);
        }
        state.current_output = None;
    }
}
