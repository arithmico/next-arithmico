use web_state::WebStateAction;

use crate::state::State;

pub struct ClearOutputAction;

impl ClearOutputAction {
    pub fn new() -> Self {
        Self
    }
}

impl WebStateAction<State> for ClearOutputAction {
    fn apply(&self, state: &mut State) {
        state.current_output = None;
    }
}
