use engine::SessionError;
use leptos::{Signal, SignalGet};

use crate::utils::expect_app_state;

pub fn use_last_output() -> Signal<Option<Result<String, SessionError>>> {
    let app_state = expect_app_state();

    Signal::derive(move || {
        app_state
            .get()
            .session
            .last_entry()
            .and_then(|statement| Some(statement.output.clone()))
    })
}
