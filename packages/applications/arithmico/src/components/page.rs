use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class=class_names!(
            "grid",
            "overflow-hidden",
            "absolute",
            "inset-0",
            "max-h-full",
            "theme-light:bg-white",
            "theme-dark:bg-neutral-900",
            "theme-light:text-black",
            "theme-dark:text-white",
            "grid-cols-[minmax(10%,auto)_1fr]"
        )>{children()}</div>
    }
}
