use leptos::prelude::*;

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
            "theme-light:bg-neutral-100",
            "theme-dark:bg-neutral-850",
            "rounded-sm",
            "border",
            "theme-light:border-neutral-200",
            "theme-dark:border-neutral-700",
            class
        )>{children()}</div>
    }
}
