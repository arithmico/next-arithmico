use yew::prelude::*;

use crate::{
    components::{Navbar, NavbarLink, Page},
    router::Route,
};

#[derive(PartialEq, Properties)]
pub struct PageWithNavbarProps {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub nav_actions: Children,
}

#[function_component]
pub fn PageWithNavbar(props: &PageWithNavbarProps) -> Html {
    html! {
        <Page class={classes!(
            "grid",
            "grid-cols-[minmax(10%,auto)_1fr]",
            "overflow-hidden",
            props.class.clone()
        )}>
            <Navbar actions={props.nav_actions.clone()}>
                <NavbarLink to={Route::Calculator}>{"Rechner"}</NavbarLink>
                <NavbarLink to={Route::Settings}>{"Einstellungen"}</NavbarLink>
                <NavbarLink to={Route::Help}>{"Hilfe"}</NavbarLink>
                <NavbarLink to={Route::About}>{"Über"}</NavbarLink>
            </Navbar>
            <main class={classes!("w-full", "pl-8", "pr-[15%]", "max-h-full", "overflow-y-auto")}>
                {props.children.clone()}
            </main>
        </Page>
    }
}
