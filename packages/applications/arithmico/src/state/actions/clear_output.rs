use web_state::StateMutation;

use crate::state::State;

pub struct ClearOutputAction;

impl ClearOutputAction {
    pub fn new() -> Self {
        Self
    }
}

impl StateMutation<State> for ClearOutputAction {
    fn apply(&self, state: &mut State) {
        state.current_output = None;
    }
}
