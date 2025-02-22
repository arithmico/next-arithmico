use leptos::prelude::*;

pub use navigation_item::NavigationItem;

mod navigation_item;

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
    view! {
        <nav class="navigation">
            <h2>Navigation</h2>
            <ul>{children()}</ul>
        </nav>
    }
}
