use engine::SessionError;
use web_state::WebStateAction;

use crate::state::State;

pub struct EvaluateAction {
    input: String,
}

impl EvaluateAction {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

impl WebStateAction<State> for EvaluateAction {
    fn apply(&self, state: &mut State) {
        let decimal_places = state.settings.decimal_places;
        let decimal_format = state.get_decimal_format();

        state
            .session
            .push(&self.input, decimal_places, decimal_format.into());
        state.current_output = state
            .session
            .last_entry()
            .and_then(|statement| Some(statement.output.clone()));
        state.current_error_trace = state
            .current_output
            .as_ref()
            .map(|output| {
                output
                    .as_ref()
                    .err()
                    .map(|session_error| match session_error {
                        SessionError::EvaluateNodeError(
                            evaluate_node_error,
                        ) => Some(evaluate_node_error.stack_trace().clone()),
                        _ => None,
                    })
            })
            .flatten()
            .flatten();
    }
}
