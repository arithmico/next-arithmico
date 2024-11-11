use leptos::*;

use crate::{app_shell::Dispatch, state::AppAction};

pub fn expect_dispatch() -> Callback<AppAction> {
    expect_context::<Dispatch>().0
}
