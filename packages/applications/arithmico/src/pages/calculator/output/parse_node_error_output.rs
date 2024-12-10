use engine::ParseNodeError;
use leptos::*;

#[component]
#[allow(unused_variables)]
pub fn ParseNodeErrorOutput(value: ParseNodeError) -> impl IntoView {
    view! { <>ParseError</> }
}
