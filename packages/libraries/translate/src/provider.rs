use leptos::*;
use translate_core::{Language, TranslationTemplateProvider};

use crate::context::TranslateContext;

#[component]
pub fn TranslateProvider(
    #[prop(into)] current_language: Signal<Language>,
    fallback_language: Language,
    children: Children,
    translations: TranslationTemplateProvider,
) -> impl IntoView {
    let context = TranslateContext::new(
        current_language,
        fallback_language,
        translations,
    );
    provide_context(context);
    view! { <>{children()}</> }
}
