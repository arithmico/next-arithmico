use leptos::{either::EitherOf3, prelude::*};
use translate_core::Translatable;

use crate::context::TranslateContext;

#[component]
pub fn TranslatableObject<T: Translatable + Send + 'static>(
    value: T,
) -> impl IntoView {
    let context = expect_context::<TranslateContext>();

    view! {
        <>
            {move || {
                let language = context.get_current_language();
                let fallback = context.get_fallback_language();
                match value.translate(language) {
                    Ok(ok) => EitherOf3::A(view! { <>{ok}</> }),
                    Err(_) => {
                        match value.translate(fallback) {
                            Ok(ok) => EitherOf3::B(view! { <>{ok}</> }),
                            Err(_) => {
                                EitherOf3::C(
                                    view! {
                                        <span class="translation-error">TranslationError</span>
                                    },
                                )
                            }
                        }
                    }
                }
            }}
        </>
    }
}
