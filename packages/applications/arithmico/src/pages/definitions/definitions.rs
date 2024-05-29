use yew::prelude::*;

use crate::{app_context::AppContext, components::*, router::Route};

#[derive(PartialEq, Properties)]
pub struct DefinitionsPageProps {}

#[function_component]
pub fn DefinitionsPage(_props: &DefinitionsPageProps) -> Html {
    let context = use_context::<AppContext>().unwrap();

    html! {
        <PageWithNavbar class={classes!("grid", "grid-rows-[auto_1fr]")}>
            <Breadcrumbs>
                <BreadcrumbsItem to={Route::Calculator}>{"Rechner"}</BreadcrumbsItem>
                <BreadcrumbsItem to={Route::Definitions}>{"Definitionen"}</BreadcrumbsItem>
            </Breadcrumbs>
            <table class={classes!(
                "w-full",
                "border-spacing-y-1",
                "border-separate"
            )}>
                {
                    context.definitions.iter().map(|(key, value)| {
                        html!(
                            <tr class={classes!(
                                "bg-white",
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
                                        "group-hover:border-neutral-500",
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
