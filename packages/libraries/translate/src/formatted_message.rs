use std::collections::HashMap;

use crate::{use_translate, IntoTranslationId};
use leptos::*;

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
                Ok(value) => view! { <>{value}</> }.into_view(),
                Err(value) => {
                    view! { <span class="bg-red-400">{value}</span> }
                        .into_view()
                }
            }}
        </>
    }
}
