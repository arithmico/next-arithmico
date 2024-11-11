use crate::utils::use_language;
use common::EvaluateNodeError;
use leptos::*;
use translate_core::Translatable;

#[component]
#[allow(unused_variables)]
pub fn EvaluateNodeErrorOutput(value: EvaluateNodeError) -> impl IntoView {
    let language = use_language();

    view! { <>{move || format!("result: {0:?}", value.translate(language.get()))}</> }
}
