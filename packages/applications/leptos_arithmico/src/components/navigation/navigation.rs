use leptos::*;

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
    view! {
        <nav>
            <h2 class="font-bold my-2 text-sm">Navigation</h2>
            <ul>{children()}</ul>
        </nav>
    }
}
