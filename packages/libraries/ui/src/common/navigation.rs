use leptos::prelude::*;

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
    view! {
        <nav class="navigation">
            <h2>Navigation</h2>
            <ul>{children()}</ul>
        </nav>
    }
}
