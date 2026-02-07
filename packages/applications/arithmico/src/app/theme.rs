use leptos::prelude::*;
use web_state::WebState;
use web_sys::MediaQueryListEvent;
use wasm_bindgen::{closure::Closure, JsCast};

use crate::state::State;

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let state = State::expect_state();

    let (prefers_dark, set_prefers_dark) = signal(false);

    Effect::new(move || {
        let media_query_list = web_sys::window()
            .and_then(|window| {
                window
                    .match_media("(prefers-color-scheme: dark)")
                    .ok()
                    .flatten()
            })
            .expect("media query should exist");

        set_prefers_dark.set(media_query_list.matches());

        let closure = Closure::wrap(Box::new(
            move |event: MediaQueryListEvent| {
                set_prefers_dark.set(event.matches());
            },
        ) as Box<dyn FnMut(_)>);

        media_query_list
            .add_event_listener_with_callback(
                "change",
                closure.as_ref().unchecked_ref(),
            )
            .expect("could register listener");

        closure.forget();
    });

    let theme = state.select(|s| s.settings.theme.clone());

    Effect::new(move |_| {
        if let Some(element) = web_sys::window().and_then(|window| {
            window
                .document()
                .unwrap()
                .document_element()
        }) {
            let theme = theme.get();
            let class = theme.resolve_html_class(*prefers_dark.read());
            element.set_class_name(class);
        }
    });

    view! {
        <div>{children()}</div>
    }
}
