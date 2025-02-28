use leptos::prelude::*;

#[component]
pub fn PageHeader(children: Children) -> impl IntoView {
    view! { <div class="page-header">{children()}</div> }
}
