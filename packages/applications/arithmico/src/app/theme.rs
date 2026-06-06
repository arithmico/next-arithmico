use leptos::prelude::*;
use web_state::WebState;
use web_sys::{
    MediaQueryListEvent,
    wasm_bindgen::{JsCast, closure::Closure},
};

use crate::state::State;

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let state = State::expect_state();

    let (prefers_dark, set_prefers_dark) = signal(false);

    if let Some(media_query_list) = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
    {
        set_prefers_dark.set(media_query_list.matches());

        let event_handler =
            Closure::wrap(Box::new(move |event: MediaQueryListEvent| {
                set_prefers_dark.set(event.matches());
            }) as Box<dyn FnMut(_)>);

        let _ = media_query_list.add_event_listener_with_callback(
            "change",
            event_handler.as_ref().unchecked_ref(),
        );

        event_handler.forget();
    }

    view! {
        <div
            data-testid="theme-div"
            class=move || {
                let theme = state.select(|state| state.settings.theme).get();
                theme.get_class(prefers_dark.get()).to_string()
            }
        >
            {children()}
        </div>
    }
}
