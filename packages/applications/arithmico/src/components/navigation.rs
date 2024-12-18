use leptos::prelude::*;

use crate::class_names;
pub use navigation_item::NavigationItem;

mod navigation_item;

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
    view! {
        <nav>
            <h2 class=class_names!(
                "my-2",
                "text-sm",
                "font-bold"
            )>Navigation</h2>
            <ul>{children()}</ul>
        </nav>
    }
}
