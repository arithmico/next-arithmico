use super::use_last_output::use_last_output;
use engine::SessionError;
use leptos::{Signal, SignalGet};
use trace::Trace;

pub fn use_error_trace() -> Signal<Option<Trace>> {
    let output = use_last_output();
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
