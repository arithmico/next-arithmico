use super::components::{CalculatorContext, CalculatorForm, CalculatorState};
use yew::prelude::*;

use yew::{function_component, html, use_reducer, Html, Properties};

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    let context = use_reducer(|| CalculatorState::new());

    html! {
        <ContextProvider<CalculatorContext> context={context}>
            <PageWithNavbar>
                <CalculatorForm />
            </PageWithNavbar>
        </ContextProvider<CalculatorContext>>
    }
}
