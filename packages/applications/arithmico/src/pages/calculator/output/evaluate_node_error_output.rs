use common::EvaluateNodeError;
use leptos::*;
use translate::TranslatableObject;

#[component]
#[allow(unused_variables)]
pub fn EvaluateNodeErrorOutput(value: EvaluateNodeError) -> impl IntoView {
    view! { <TranslatableObject value /> }
}
