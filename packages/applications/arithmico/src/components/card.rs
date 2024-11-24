use leptos::*;

use crate::class_names;

#[component]
pub fn Card(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=class_names!(
            "theme-light:text-black",
            "theme-dark:text-white",
            "theme-light:bg-white",
            "theme-dark:bg-neutral-800",
            "rounded-md",
            "border",
            "border-neutral-200",
            class
        )>{children()}</div>
    }
}
