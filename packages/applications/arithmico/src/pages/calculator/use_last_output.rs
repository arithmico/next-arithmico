use engine::SessionError;
use leptos::prelude::Signal;
use web_state::WebState;

use crate::state::State;

pub fn use_last_output() -> Signal<Option<Result<String, SessionError>>> {
    let state = State::use_state();

    state.select(|state| {
        state
            .session
            .last_entry()
            .and_then(|statement| Some(statement.output.clone()))
    })
}
