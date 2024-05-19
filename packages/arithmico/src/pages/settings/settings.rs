use yew::prelude::*;

use crate::components::*;

use super::components::{
    decimal_places_selector::DecimalPlacesSelector,
    interface_language_selector::InterfaceLanguageSelector,
};

#[derive(PartialEq, Properties)]
pub struct SettingsPageProps {}

#[function_component]
pub fn SettingsPage(_props: &SettingsPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <section>
                <h1 class={classes!("text-3xl", "font-medium", "mt-8")}>
                    {"Bedienoberfläche"}
                </h1>
                <ul class={classes!("pl-8")}>
                    <li class={classes!("pt-2")}>
                        <InterfaceLanguageSelector />
                    </li>
                    <li class={classes!("pt-2")}>
                        <DecimalPlacesSelector />
                    </li>
                </ul>
            </section>
        </PageWithNavbar>
    }
}
