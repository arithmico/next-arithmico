use leptos::*;
use translate_core::Translatable;

use crate::context::TranslateContext;

#[component]
pub fn TranslatableObject<T: Translatable + 'static>(
    value: T,
) -> impl IntoView {
    let context = expect_context::<TranslateContext>();

    view! {
        <>
            {move || {
                let language = context.get_current_language();
                let fallback = context.get_fallback_language();
                match value.translate(language) {
                    Ok(ok) => {

                        view! { <>{ok}</> }
                            .into_view()
                    }
                    Err(_) => {
                        match value.translate(fallback) {
                            Ok(ok) => view! { <>{ok}</> }.into_view(),
                            Err(_) => {
                                view! {
                                    <span class="bg-red-400 border border-red-700">
                                        TranslationError
                                    </span>
                                }
                                    .into_view()
                            }
                        }
                    }
                }
            }}
        </>
    }
}
