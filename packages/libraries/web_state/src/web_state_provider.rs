use leptos::prelude::*;

use crate::WebState;

#[derive(Clone)]
pub struct WebStateWrapper<S: WebState>(pub S);

#[component]
pub fn WebStateProvider<S: WebState>(
    state: S,
    children: Children,
) -> impl IntoView {
    provide_context(RwSignal::new(WebStateWrapper(state)));
    view! { <>{children()}</> }
}
