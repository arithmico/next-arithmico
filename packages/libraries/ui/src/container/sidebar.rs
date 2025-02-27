use leptos::prelude::*;

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h1>{title}</h1>
            {children()}
        </aside>
    }
}
