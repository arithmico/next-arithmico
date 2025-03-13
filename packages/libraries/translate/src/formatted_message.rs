use std::collections::HashMap;

use crate::{IntoTranslationId, use_translate};
use leptos::{either::Either, prelude::*};

#[component]
pub fn FormattedMessage(
    id: impl IntoTranslationId + 'static,
    #[prop(default = Signal::derive(||HashMap::new()))] keys: Signal<
        HashMap<String, String>,
    >,
) -> impl IntoView {
    let id = id.into_translation_id().to_string();
    let translate = use_translate();

    view! {
        <>
            {move || match translate(&id, Some(keys.get())) {
                Ok(value) => Either::Left(view! { <>{value}</> }),
                Err(value) => {
                    Either::Right(
                        view! { <span class="translation-error">{value}</span> },
                    )
                }
            }}
        </>
    }
}
