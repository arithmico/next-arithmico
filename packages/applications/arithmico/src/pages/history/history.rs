use yew::prelude::*;

use crate::{app_context::AppContext, components::*, router::Route};

#[derive(PartialEq, Properties)]
pub struct HistoryPageProps {}

#[function_component]
pub fn HistoryPage(_props: &HistoryPageProps) -> Html {
    let context = use_context::<AppContext>().unwrap();

    html! {
        <PageWithNavbar class={classes!("grid", "grid-rows-[auto_1fr]")}>
            <Breadcrumbs>
                <BreadcrumbsItem to={Route::Calculator}>{"Rechner"}</BreadcrumbsItem>
                <BreadcrumbsItem to={Route::History}>{"Verlauf"}</BreadcrumbsItem>
            </Breadcrumbs>
            <ul class={classes!(
                "flex",
                "flex-col",
                "gap-1",
                "mb-2",
            )}>
                {
                    context.session.get_statements().iter().map(|statement| {
                        html!(
                            <li class={classes!(
                                "flex",
                                "flex-col",
                                "bg-white",
                                "border",
                                "border-neutral-300",
                                "hover:border-neutral-500",
                                "rounded-sm",
                                "p-2"
                            )}>
                                <dl class={classes!(
                                    "grid",
                                    "grid-cols-[auto_1fr]",
                                    "gap-x-4",
                                    "gap-y-2"
                                )}>
                                    <dt class={classes!("text-right", "text-black/50")}>{"Eingabe"}</dt>
                                    <dd>{statement.input.clone()}</dd>
                                    <dt class={classes!("text-right", "text-black/50")}>{"Ausgabe"}</dt>
                                    <dd>{match &statement.output {
                                        Ok(value) => value.clone(),
                                        Err(error) => error.clone().to_string(),
                                    }}</dd>
                                </dl>
                            </li>
                        )
                    }).collect::<Html>()
                }
            </ul>
        </PageWithNavbar>
    }
}
