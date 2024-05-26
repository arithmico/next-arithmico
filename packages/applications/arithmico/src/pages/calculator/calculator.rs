use super::components::{Form, Toolbar};
use yew::prelude::*;

use crate::components::{PageTitle, PageWithNavbar};

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    html! {
        <PageWithNavbar nav_actions={html!(
            <>
                <h2 class={classes!(
                    "font-bold",
                    "my-2",
                    "text-sm",
                    "mt-4"
                )}>{"Aktionen"}</h2>
                <Toolbar />
            </>
        )}>
            <div class={classes!("grid", "grid-rows-[auto_1fr]", "w-full", "h-full")}>
                <PageTitle>{"Rechner"}</PageTitle>
                <div class={classes!("py-8", "flex", "flex-col", "w-full", "justify-center")}>
                    <Form />
                </div>
            </div>
        </PageWithNavbar>
    }
}
