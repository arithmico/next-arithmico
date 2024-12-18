use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! {
        <h1 class=class_names!(
            "py-4",
            "pr-10",
            "text-2xl",
            "font-medium",
            "theme-light:text-black",
            "theme-dark:text-white"
        )>{children()}</h1>
    }
}
