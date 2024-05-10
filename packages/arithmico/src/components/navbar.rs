use yew::{classes, function_component, html, Classes, Html, Properties};
use yew_router::components::Link;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Html,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html!(
        <nav class={classes!("w-full", "flex", "justify-center", "items-center", "bg-blue-400")}>
            <h1 class="pr-4">{"Arithmico"}</h1>
            <ul class="flex">
                {props.children.clone()}
            </ul>
        </nav>
    )
}

#[derive(PartialEq, Properties)]
pub struct NavbarLinkProps {
    pub to: Route,
    pub children: Html,
}

#[function_component]
pub fn NavbarLink(props: &NavbarLinkProps) -> Html {
    html! {
        <li class={classes!("flex")}>
            <LinkWrapper
                class={classes!("p-2")}
                to={props.to.clone()}
            >
                {props.children.clone()}
            </LinkWrapper>
        </li>
    }
}

#[derive(PartialEq, Properties)]
struct LinkWrapperProps {
    pub to: Route,
    pub class: Classes,
    pub children: Html,
}

#[function_component]
fn LinkWrapper(props: &LinkWrapperProps) -> Html {
    html! {
        <Link<Route>
            classes={props.class.clone()}
            to={props.to.clone()}
        >{props.children.clone()}</Link<Route>>
    }
}
