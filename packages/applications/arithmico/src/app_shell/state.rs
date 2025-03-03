use leptos::prelude::*;
use web_state::WebStateProvider;

use crate::state::State;

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let state = State::new();

    view! { <WebStateProvider state=state>{children()}</WebStateProvider> }
}
