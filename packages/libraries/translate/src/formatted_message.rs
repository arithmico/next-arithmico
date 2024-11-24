use std::collections::HashMap;

use crate::context::TranslateContext;
use leptos::*;

#[component]
pub fn FormattedMessage(
    #[prop(into)] id: String,
    #[prop(default = Signal::derive(||HashMap::new()))] keys: Signal<
        HashMap<String, String>,
    >,
) -> impl IntoView {
    let context = expect_context::<TranslateContext>();
    let content: Signal<Result<String, &str>> = Signal::derive(move || {
        let keys = keys.get();
        let template = context
            .get_template_provider()
            .get_template(&id)
            .ok_or("TranslationError")?;

        match template.translate_with(context.get_current_language(), &keys) {
            Ok(ok) => Ok(ok),
            Err(_) => match template
                .translate_with(context.get_fallback_language(), &keys)
            {
                Ok(ok) => Ok(ok),
                Err(_) => Err("TranslationError"),
            },
        }
    });

    view! {
        <>
            {move || match content.get() {
                Ok(value) => view! { <>{value}</> }.into_view(),
                Err(value) => view! { <span class="bg-red-400">{value}</span> }.into_view(),
            }}
        </>
    }
}
