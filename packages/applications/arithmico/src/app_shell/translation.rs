use crate::utils::use_app_state;
use common::Language;
use leptos::*;
use translate::{TranslateProvider as Provider, TranslationTemplateProvider};

#[component]
pub fn TranslationProvider(children: Children) -> impl IntoView {
    let state = use_app_state();
    let current_language =
        Signal::derive(move || state.get().settings.language);
    let translations = TranslationTemplateProvider::try_from_toml(
        include_str!("../../translations.toml"),
    )
    .expect("translations");

    view! {
        <Provider current_language fallback_language=Language::English translations>
            {children()}
        </Provider>
    }
}
