use leptos::*;

use crate::state::AppState;

pub fn expect_app_state() -> ReadSignal<AppState> {
    expect_context::<ReadSignal<AppState>>()
}
