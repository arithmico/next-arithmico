use yew::{function_component, html, Html, Properties};

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct SettingsPageProps {}

#[function_component]
pub fn SettingsPage(_props: &SettingsPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <p>{"settings"}</p>
        </PageWithNavbar>
    }
}
