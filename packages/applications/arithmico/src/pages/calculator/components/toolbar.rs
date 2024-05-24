use yew::prelude::*;

use crate::{components::Link, router::Route};

#[derive(PartialEq, Properties)]
pub struct ToolbarProps {}

#[function_component]
pub fn Toolbar(props: &ToolbarProps) -> Html {
    let ToolbarProps {} = props;
    html! {
        <div class={classes!("w-full", "grid-cols-3", "grid", "gap-2", "mt-8")}>
            <button class={classes!(
                "bg-neutral-200",
                "p-3",
                "rounded-sm",
                "cursor-pointer",
                "hover:bg-neutral-300",
            )}>
                {"Definitionen"}
            </button>
            <Link
                class={classes!(
                    "bg-neutral-200",
                    "p-3",
                    "rounded-sm",
                    "cursor-pointer",
                    "hover:bg-neutral-300",
                    "text-center"
                )}
                to={Route::History}
            >
                {"Verlauf"}
            </Link>
            <button class={classes!(
                "bg-neutral-200",
                "p-3",
                "rounded-sm",
                "cursor-pointer",
                "hover:bg-neutral-300",
            )}>
                {"Zurücksetzen"}
            </button>
        </div>
    }
}
