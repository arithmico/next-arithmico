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
            <PageTitle>{"Einstellungen"}</PageTitle>
            <section>
                <h2 class={classes!("text-xl", "font-medium", "mt-8")}>
                    {"Bedienoberfläche"}
                </h2>
                <ul class={classes!("pl-8")}>
                    <li class={classes!("pt-2")}>
                        <InterfaceLanguageSelector />
                    </li>
                </ul>
            </section>
            <section>
                <h2 class={classes!("text-xl", "font-medium", "mt-8")}>
                    {"Mathematik"}
                </h2>
                <ul class={classes!("pl-8")}>
                    <li class={classes!("pt-2")}>
                        <DecimalPlacesSelector />
                    </li>
                </ul>
            </section>
        </PageWithNavbar>
    }
}
