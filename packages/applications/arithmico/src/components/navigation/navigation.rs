use leptos::*;

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
    view! {
        <nav>
            <h2 class="my-2 text-sm font-bold">Navigation</h2>
            <ul>{children()}</ul>
        </nav>
    }
}
