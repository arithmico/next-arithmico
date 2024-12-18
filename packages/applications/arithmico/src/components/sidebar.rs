use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    view! {
        <aside class=class_names!(
            "flex",
            "flex-col",
            "px-2",
            "h-screen",
            "theme-light:text-black",
            "theme-dark:text-white",
            "theme-light:bg-neutral-100",
            "theme-dark:bg-neutral-850",
            "border-r",
            "theme-light:border-neutral-200",
            "theme-dark:border-neutral-700"
        )>
            <h1 class="py-4 pr-10 text-2xl font-light">Arithmico</h1>
            {children()}
        </aside>
    }
}
