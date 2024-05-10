use yew::{function_component, html, Html, Properties};

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct HelpPageProps {}

#[function_component]
pub fn HelpPage(_props: &HelpPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <p>{"help"}</p>
        </PageWithNavbar>
    }
}
