use leptos::*;
use leptos_router::A;
use translate::FormattedMessage;

use crate::{
    class_names,
    components::{HistoryIcon, ListIcon, MenuIcon, PageTitle},
};

#[component]
pub fn CalculatorHeader() -> impl IntoView {
    view! {
        <div class=class_names!("flex", "items-center")>
            <PageTitle>
                <FormattedMessage id="calculator.title" />
            </PageTitle>
            <div class=class_names!(
                "flex",
                "ml-auto",
                "gap-2",
                "items-center"
            )>
                <ToolbarNavigation>
                    <ToolbarNavigationItem to="/history">
                        <HistoryIcon class=class_names!(
                            "w-5",
                            "h-5",
                            "rotate-180",
                            "theme-dark:fill-white/50",
                            "theme-dark:group-hover:fill-white"
                        ) />
                        <FormattedMessage id="calculator.toolbar.history" />
                    </ToolbarNavigationItem>
                    <ToolbarNavigationItem to="/definitions">
                        <ListIcon class=class_names!(
                            "w-5",
                            "h-5",
                            "rotate-180",
                            "theme-dark:fill-white/50",
                            "theme-dark:group-hover:fill-white"
                        ) />
                        <FormattedMessage id="calculator.toolbar.definitions" />
                    </ToolbarNavigationItem>
                </ToolbarNavigation>

                <button class=class_names!(
                    "theme-dark:text-white/75",
                    "theme-dark:border-neutral-700",
                    "theme-dark:bg-neutral-850",
                    "theme-dark:hover:border-neutral-500",
                    "theme-dark:hover:bg-neutral-800",
                    "theme-dark:hover:text-white",
                    "hover:cursor-pointer",
                    "border",
                    "py-1",
                    "pl-3",
                    "pr-2",
                    "text-xs",
                    "rounded-sm",
                    "flex",
                    "items-center",
                    "gap-1",
                    "group"
                )>
                    <FormattedMessage id="calculator.toolbar.actions" />
                    <MenuIcon class=class_names!(
                        "w-5",
                        "h-5",
                        "rotate-180",
                        "theme-dark:fill-white/50",
                        "theme-dark:group-hover:fill-white"
                    ) />
                </button>

            </div>
        </div>
    }
}

#[component]
fn ToolbarNavigation(children: Children) -> impl IntoView {
    view! {
        <nav class="flex">
            <ul class=class_names!("flex", "gap-2")>{children()}</ul>
        </nav>
    }
}

#[component]
fn ToolbarNavigationItem(
    children: Children,
    to: impl ToString,
) -> impl IntoView {
    view! {
        <li class="flex">
            <A
                class=class_names!(
                    "theme-dark:text-white/75",
                    "theme-dark:border-neutral-700",
                    "theme-dark:bg-neutral-850",
                    "theme-dark:hover:border-neutral-500",
                    "theme-dark:hover:bg-neutral-800",
                    "theme-dark:hover:text-white",
                    "hover:cursor-pointer",
                    "border",
                    "py-1",
                    "pr-3",
                    "pl-2",
                    "text-xs",
                    "rounded-sm",
                    "flex",
                    "gap-2",
                    "items-center",
                    "group"
                )
                href=to.to_string()
            >
                {children()}
            </A>
        </li>
    }
}
