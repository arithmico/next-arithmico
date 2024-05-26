use yew::prelude::*;

use icons::back::BackIcon;

use crate::{
    app_context::AppContext,
    components::{Link, PageWithNavbar},
    router::Route,
};

#[derive(PartialEq, Properties)]
pub struct DefinitionsPageProps {}

#[function_component]
pub fn DefinitionsPage(_props: &DefinitionsPageProps) -> Html {
    let context = use_context::<AppContext>().unwrap();

    html! {
        <PageWithNavbar class={classes!("grid", "grid-rows-[auto_1fr]")}>
            <div class={classes!(
                "grid",
                "grid-cols-[auto_1fr]",
                "gap-2",
                "bg-neutral-200",
                "p-2",
                "my-4",
                "rounded-sm",
                "items-center"
            )}>
                <h1 class={classes!("text-xl", "font-light")}>{"Definitionen"}</h1>
                <Link
                    to={Route::Calculator}
                    class={classes!(
                        "col-start-1",
                        "row-start-1",
                        "flex",
                        "items-center",
                        "justify-center",
                        "hover:bg-neutral-300",
                        "p-2",
                        "rounded-sm",
                        "group"
                    )}
                >
                    <span class={classes!("sr-only")}>{"Zurück"}</span>
                    <BackIcon class={classes!("fill-black/50", "group-hover:fill-black")} />
                </Link>
            </div>
            <table class={classes!(
                "w-full",
                "border-spacing-y-1",
                "border-separate"
            )}>
                {
                    context.definitions.iter().map(|(key, value)| {
                        html!(
                            <tr class={classes!(
                                "bg-neutral-100",
                                "hover:bg-neutral-200",
                                "rounded-sm",
                                "group",
                            )}>
                                <td class={
                                    classes!(
                                        "text-right",
                                        "after:content-[':=']",
                                        "py-3",
                                        "pr-0",
                                        "pl-4",
                                        "border",
                                        "border-r-0",
                                        "border-neutral-300",
                                        "group-hover:border-neutral-400",
                                    )}
                                >
                                    <span class={classes!("pr-4")}>
                                        {key.clone()}
                                    </span>
                                </td>
                                <td class={classes!(
                                    "px-4",
                                    "py-3",
                                    "border",
                                    "border-l-0",
                                    "border-neutral-300",
                                    "group-hover:border-neutral-400",
                                    "w-full"
                                )}>
                                    {value.clone()}
                                </td>
                            </tr>
                        )
                    }).collect::<Html>()
                }
            </table>
        </PageWithNavbar>
    }
}
