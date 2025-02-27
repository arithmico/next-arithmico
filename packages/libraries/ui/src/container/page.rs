use leptos::prelude::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! { <div class="page">{children()}</div> }
}
