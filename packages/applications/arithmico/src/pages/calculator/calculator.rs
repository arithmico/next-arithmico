use super::components::{Form, Toolbar};
use yew::prelude::*;

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <div class={classes!("grid", "grid-rows-[1fr_auto]", "w-full", "h-full", "p-4")}>
                <div class={classes!("py-8", "flex", "flex-col", "w-full", "justify-center")}>
                    <Form />
                </div>
                <Toolbar />
            </div>
        </PageWithNavbar>
    }
}
