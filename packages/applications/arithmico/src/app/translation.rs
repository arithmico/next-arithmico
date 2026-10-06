use common::Language;
use js_sys::wasm_bindgen::{JsCast, prelude::Closure};
use leptos::{logging::warn, prelude::*};
use leptos_meta::{Html, provide_meta_context};
use translate::{TranslateProvider as Provider, TranslationTemplateProvider};
use web_state::WebState;
use web_sys::Event;

use crate::state::State;

#[component]
pub fn TranslationProvider(children: Children) -> impl IntoView {
    let state = State::expect_state();
    let browser_language_trigger = Trigger::new();
    let language = state.select(|state| state.settings.get_language());
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

    let current_language = Signal::derive(move || {
        browser_language_trigger.track();
        language.get()
    });

    event_handler.forget();

    provide_meta_context();

    view! {
        <Html
            {..}
            lang=move || match current_language.get() {
                Language::German => "de",
                Language::English => "en",
            }
        />
        <Provider
            current_language=current_language
            fallback_language=Language::English
            translations
        >
            {children()}
        </Provider>
    }
}
