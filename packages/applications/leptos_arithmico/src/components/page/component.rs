use leptos::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! { <div class="absolute inset-0">{children()}</div> }
}
