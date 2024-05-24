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
}

#[function_component]
pub fn PageWithNavbar(props: &PageWithNavbarProps) -> Html {
    html! {
        <Page class={props.class.clone()}>
            <Navbar>
                <NavbarLink to={Route::Calculator}>{"Rechner"}</NavbarLink>
                <NavbarLink to={Route::Settings}>{"Einstellungen"}</NavbarLink>
                <NavbarLink to={Route::Help}>{"Hilfe"}</NavbarLink>
                <NavbarLink to={Route::About}>{"Über"}</NavbarLink>
            </Navbar>
            <main class={classes!("w-full", "px-[20%]", "max-h-full", "overflow-y-auto")}>
                {props.children.clone()}
            </main>
        </Page>
    }
}
