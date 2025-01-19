use leptos::prelude::*;

use crate::class_names;

#[component]
pub fn NavigationItem(children: Children, to: String) -> impl IntoView {
    view! {
        <li class="flex flex-col">
            <a
                class=class_names!(
                    "flex",
                    "items-center",
                    "py-2",
                    "px-6",
                    "rounded-md",
                    "theme-light:hover:bg-neutral-200",
                    "theme-dark:hover:bg-neutral-800"
                )
                href=to
            >
                {children()}
            </a>
        </li>
    }
}
