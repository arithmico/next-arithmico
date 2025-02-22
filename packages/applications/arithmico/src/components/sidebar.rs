use leptos::prelude::*;

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h1>Arithmico</h1>
            {children()}
        </aside>
    }
}
