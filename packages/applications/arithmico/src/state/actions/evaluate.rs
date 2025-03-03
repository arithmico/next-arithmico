use common::EvaluateNodeOptions;
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
        let options = EvaluateNodeOptions::new(
            (&state.settings.decimal_places).into(),
            state
                .settings
                .override_decimal_format
                .decimal_format()
                .cloned()
                .unwrap_or_else(|| (&state.settings.language).into()),
        );
        state.session.push(&self.input, &options);
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
                        ) => evaluate_node_error.stack_trace().first().cloned(),
                        _ => None,
                    })
            })
            .flatten()
            .flatten();
    }
}
