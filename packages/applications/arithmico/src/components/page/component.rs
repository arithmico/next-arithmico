use leptos::*;

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
            "theme-light:bg-neutral-100",
            "theme-dark:bg-neutral-900",
            "grid-cols-[minmax(10%,auto)_1fr]"
        )>{children()}</div>
    }
}
