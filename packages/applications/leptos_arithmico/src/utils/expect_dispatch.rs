use leptos::*;

use crate::state::{AppAction, Dispatch};

pub fn expect_dispatch() -> Callback<AppAction> {
    expect_context::<Dispatch>().0
}
