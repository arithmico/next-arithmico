use crate::{class_names, components::*, utils::use_app_state};
use common::Language;
use leptos::prelude::*;
use translate::{use_translate, FormattedMessage};

#[component]
pub fn ReferencePage() -> impl IntoView {
    let app_state = use_app_state();
    let translate = use_translate();

    view! {
        <PageWithSidebar>
            <div class=class_names!("flex", "items-center")>
                <PageTitle>
                    <FormattedMessage id="reference.title" />
                </PageTitle>
                <input
                    type="search"
                    placeholder=translate("reference.search", None)
                        .unwrap_or(String::from("TranslationError"))
                    class=class_names!(
                        "border",
                        "py-2",
                        "pr-3",
                        "pl-2",
                        "text-xs",
                        "rounded-sm",
                        "w-full",
                        "outline-none",
                        "theme-dark:text-white",
                        "theme-dark:border-neutral-700",
                        "theme-dark:bg-neutral-850",
                    )
                />

            </div>
            {move || {
                app_state
                    .get()
                    .session
                    .documentation()
                    .modules()
                    .iter()
                    .map(|module| {
                        view! {
                            <details
                                open
                                class=class_names!(
                                    "theme-light:bg-neutral-100",
                                    "theme-dark:bg-neutral-850",
                                    "theme-dark:border-white/5",
                                    "theme-light:border-black/5",
                                    "rounded-sm",
                                    "border-2"
                                )
                            >
                                <summary class=class_names!(
                                    "list-none",
                                    "p-2",
                                    "rounded-sm",
                                    "theme-dark:bg-neutral-800",
                                    "theme-light:bg-neutral-200"
                                )>{module.name(&Language::English)}</summary>
                                <dl class=class_names!(
                                    "grid",
                                    "grid-cols-[1fr_3fr]",
                                    "p-2"
                                )>
                                    {module
                                        .items()
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <>
                                                    <dt class=class_names!(
                                                        "py-2"
                                                    )>{item.synopsis(&Language::English)}</dt>
                                                    <dd class=class_names!(
                                                        "py-2"
                                                    )>{item.description(&Language::English)}</dd>
                                                </>
                                            }
                                        })
                                        .collect_view()}
                                </dl>
                            </details>
                        }
                    })
                    .collect_view()
            }}

        </PageWithSidebar>
    }
}
