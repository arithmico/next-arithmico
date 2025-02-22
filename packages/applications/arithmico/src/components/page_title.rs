use leptos::prelude::*;

#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! { <h1 class="page-title">{children()}</h1> }
}
