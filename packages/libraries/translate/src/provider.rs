use leptos::*;
use translate_core::Language;

use crate::context::TranslateContext;

#[component]
pub fn TranslateProvider(
    #[prop(into)] current_language: Signal<Language>,
    fallback_language: Language,
    children: Children,
) -> impl IntoView {
    let context = TranslateContext::new(current_language, fallback_language);
    provide_context(context);
    view! { <>{children()}</> }
}
