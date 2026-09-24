use common::Language;
use js_sys::wasm_bindgen::{JsCast, prelude::Closure};
use leptos::{logging::warn, prelude::*};
use translate::{TranslateProvider as Provider, TranslationTemplateProvider};
use web_state::WebState;
use web_sys::Event;

use crate::state::State;

#[component]
pub fn TranslationProvider(children: Children) -> impl IntoView {
    let state = State::expect_state();
    let browser_language_trigger = Trigger::new();
    let current_language = state.select(|state| state.settings.get_language());
    let translations = TranslationTemplateProvider::try_from_toml(
        include_str!("../../translations.toml"),
    )
    .expect("translations");

    let event_handler = Closure::wrap(Box::new(move |_: Event| {
        browser_language_trigger.notify();
    }) as Box<dyn FnMut(_)>);

    if let Err(err) = window().add_event_listener_with_callback(
        "languagechange",
        event_handler.as_ref().unchecked_ref(),
    ) {
        warn!(
            "Failed to register language change event handler: {:?}",
            err
        );
    }

    event_handler.forget();

    view! {
        <Provider
            current_language=Signal::derive(move || {
                browser_language_trigger.track();
                current_language.get()
            })
            fallback_language=Language::English
            translations
        >
            {children()}
        </Provider>
    }
}
