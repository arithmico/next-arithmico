use yew::{classes, function_component, html, Html, Properties};

use crate::{
    components::{Navbar, NavbarLink, Page},
    router::Route,
};

#[derive(PartialEq, Properties)]
pub struct PageWithNavbarProps {
    pub children: Html,
}

#[function_component]
pub fn PageWithNavbar(props: &PageWithNavbarProps) -> Html {
    html! {
        <Page>
            <Navbar>
                <NavbarLink to={Route::Calculator}>{"Rechner"}</NavbarLink>
                <NavbarLink to={Route::Settings}>{"Einstellungen"}</NavbarLink>
                <NavbarLink to={Route::Help}>{"Hilfe"}</NavbarLink>
                <NavbarLink to={Route::About}>{"Über"}</NavbarLink>
            </Navbar>
            <main class={classes!("w-full", "px-[20%]")}>
                {props.children.clone()}
            </main>
        </Page>
    }
}
