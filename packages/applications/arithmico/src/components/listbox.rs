use common_ui::form::listbox::{
    Listbox as HeadlessListbox, ListboxButton as HeadlessListboxButton,
    ListboxOption as HeadlessListboxOption,
    ListboxOptions as HeadlessListboxOptions,
};
use leptos::*;

use crate::class_names;

#[component]
pub fn Listbox<T>(
    children: Children,
    #[prop(into)] on_change: Callback<T>,
    #[prop(into)] value: Signal<T>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView
where
    T: PartialEq + Clone + 'static,
{
    view! {
        <HeadlessListbox
            value
            class=class_names!(
                "flex",
                "relative",
                "flex-col",
                "ml-auto",
                "w-32",
                class.unwrap_or(String::new())
            )
            on_change
            children
        />
    }
}

#[component(transparent)]
pub fn ListboxOptions(
    children: Children,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <HeadlessListboxOptions
            children
            class=class_names!(
                "absolute",
                "z-10",
                "mt-1",
                "w-full",
                "border",
                "theme-light:bg-neutral-200",
                "theme-dark:bg-neutral-700",
                "border-neutral-300",
                "rounded-sm",
                class.unwrap_or(String::new())
            )
        />
    }
}

#[component(transparent)]
pub fn ListboxButton(
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <HeadlessListboxButton
            children
            class=class_names!(
                "py-1",
                "px-2",
                "text-left",
                "rounded-sm",
                "border",
                "theme-light:bg-neutral-200",
                "theme-dark:bg-neutral-700",
                "border-neutral-300",
                "theme-light:hover:bg-neutral-300",
                "theme-dark:hover:bg-neutral-600",
                class.unwrap_or(String::new())
            )
        />
    }
}

#[component(transparent)]
pub fn ListboxOption<T: PartialEq + Clone + 'static>(
    value: T,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <HeadlessListboxOption
            value
            children
            class=|selected: bool| {
                let class = class_names!(
                    "px-2",
                    "py-1",
                    "theme-light:hover:bg-neutral-300",
                    "theme-dark:hover:bg-neutral-600",
                    "rounded-sm",
                    "focus-visible:outline-2",
                    "outline-black"
                );
                format!(
                    "{} {}",
                    class,
                    if selected { "font-bold" } else { "font-normal" },
                )
            }
        />
    }
}
