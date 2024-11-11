use leptos::*;

use crate::state::AppState;

pub fn use_app_state() -> ReadSignal<AppState> {
    expect_context::<ReadSignal<AppState>>()
}
