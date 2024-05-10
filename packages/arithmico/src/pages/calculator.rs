use yew::{function_component, html, Html, Properties};

use crate::components::{Navbar, Page};

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    html! {
        <Page>
            <Navbar>
                <li class="p-2">{"Rechner"}</li>
                <li class="p-2">{"Einstellungen"}</li>
                <li class="p-2">{"Hilfe"}</li>
                <li class="p-2">{"Über"}</li>
            </Navbar>
        </Page>
    }
}
