use common::Language;
use leptos::prelude::*;
use translate::{TranslateProvider as Provider, TranslationTemplateProvider};
use web_state::WebState;

use crate::state::State;

#[component]
pub fn TranslationProvider(children: Children) -> impl IntoView {
    let state = State::use_state();
    let current_language = state.select(|state| state.settings.language);
    let translations = TranslationTemplateProvider::try_from_toml(
        include_str!("../../translations.toml"),
    )
    .expect("translations");

    view! {
        <Provider
            current_language
            fallback_language=Language::English
            translations
        >
            {children()}
        </Provider>
    }
}
