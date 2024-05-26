use yew::{function_component, html, Html, Properties};

use crate::components::*;

#[derive(PartialEq, Properties)]
pub struct AboutPageProps {}

#[function_component]
pub fn AboutPage(_props: &AboutPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <PageTitle>{"Über"}</PageTitle>
        </PageWithNavbar>
    }
}
