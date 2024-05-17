use super::components::CalculatorForm;
use yew::prelude::*;

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <CalculatorForm />
        </PageWithNavbar>
    }
}
