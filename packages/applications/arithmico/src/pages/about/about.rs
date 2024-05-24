use yew::{function_component, html, Html, Properties};

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct AboutPageProps {}

#[function_component]
pub fn AboutPage(_props: &AboutPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <p>{"about"}</p>
        </PageWithNavbar>
    }
}
