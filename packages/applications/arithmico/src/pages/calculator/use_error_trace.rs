use engine::SessionError;
use leptos::prelude::{Get, Signal};
use trace::Trace;
use web_state::WebState;

use crate::state::State;

pub fn use_error_trace() -> Signal<Option<Trace>> {
    let state = State::expect_state();
    let output = state.select(|state| state.current_output);

    Signal::derive(move || {
        output
            .get()
            .map(|output| {
                output.err().map(|session_error| match session_error {
                    SessionError::EvaluateNodeError(evaluate_node_error) => {
                        evaluate_node_error.stack_trace().first().cloned()
                    }
                    _ => None,
                })
            })
            .flatten()
            .flatten()
    })
}
