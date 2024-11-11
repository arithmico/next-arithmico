use crate::utils::use_app_state;
use common::Language;
use leptos::*;
use translate::TranslateProvider as Provider;

#[component]
pub fn TranslationProvider(children: Children) -> impl IntoView {
    let state = use_app_state();
    let current_language =
        Signal::derive(move || state.get().settings.language);

    view! {
        <Provider current_language fallback_language=Language::English>
            {children()}
        </Provider>
    }
}
