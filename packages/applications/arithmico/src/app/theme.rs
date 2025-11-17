use leptos::prelude::*;
use web_state::WebState;

use crate::state::State;

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let state = State::expect_state();

    view! {
        <div class=move || {
            state.select(|state| state.settings.theme.get_class().to_string())
        }>{children()}</div>
    }
}
