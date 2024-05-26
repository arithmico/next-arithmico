use yew::prelude::*;

use crate::{components::Link, router::Route};

#[derive(PartialEq, Properties)]
pub struct ToolbarProps {}

#[function_component]
pub fn Toolbar(props: &ToolbarProps) -> Html {
    let ToolbarProps {} = props;
    html! {
        <div class={classes!("w-full", "flex", "flex-col", "gap-1")}>
            <Link class={classes!(
                    "py-2",
                    "px-4",
                    "hover:bg-neutral-200",
                    "flex",
                    "items-center",
                    "rounded-md",
                )}
                to={Route::Definitions}
            >
                {"Definitionen"}
            </Link>
            <Link
                class={classes!(
                    "py-2",
                    "px-4",
                    "hover:bg-neutral-200",
                    "flex",
                    "items-center",
                    "rounded-md",
                )}
                to={Route::History}
            >
                {"Verlauf"}
            </Link>
            <button class={classes!(
                "py-2",
                "px-4",
                "hover:bg-neutral-200",
                "flex",
                "items-center",
                "rounded-md",
            )}>
                {"Zurücksetzen"}
            </button>
        </div>
    }
}
