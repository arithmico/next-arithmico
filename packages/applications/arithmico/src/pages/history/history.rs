use yew::prelude::*;

use crate::{app_context::AppContext, components::PageWithNavbar};

#[derive(PartialEq, Properties)]
pub struct HistoryPageProps {}

#[function_component]
pub fn HistoryPage(_props: &HistoryPageProps) -> Html {
    let context = use_context::<AppContext>().unwrap();

    html! {
        <PageWithNavbar class={classes!("grid", "grid-rows-[auto_1fr]")}>
            <div class={classes!(
                "flex",
                "bg-neutral-100",
                "p-2",
                "my-4",
                "rounded-sm"
            )}>
                <h1 class={classes!("text-3xl", "font-light")}>{"Verlauf"}</h1>
            </div>
            <ul class={classes!(
                "flex",
                "flex-col",
                "gap-2",
            )}>
                {
                    context.session.get_statements().iter().map(|statement| {
                        html!(
                            <li class={classes!(
                                "flex",
                                "flex-col",
                                "bg-neutral-100",
                                "hover:bg-neutral-200",
                                "rounded-sm",
                                "p-2"
                            )}>
                                <dl class={classes!(
                                    "grid",
                                    "grid-cols-[auto_1fr]",
                                    "gap-x-4",
                                    "gap-y-2"
                                )}>
                                    <dt class={classes!("text-right")}>{"Eingabe"}</dt>
                                    <dd>{statement.input.clone()}</dd>
                                    <dt class={classes!("text-right")}>{"Ausgabe"}</dt>
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
